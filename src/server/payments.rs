use reqwest::Client;
use serde::Deserialize;
use tracing::{error, instrument};

#[derive(Debug, Deserialize)]
struct PaymentResponse {
    success: bool,
    transaction_id: String,
}

/// Process payment by calling an external API.
#[instrument(skip(client, amount, card_token))]
pub async fn process_payment(
    client: &Client,
    amount: f64,
    card_token: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    let resp = client
        .post("https://api.paymentprovider.com/charge")
        .json(&serde_json::json!({
            "amount": amount,
            "currency": "USD",
            "token": card_token
        }))
        .send()
        .await?;

    if !resp.status().is_success() {
        error!(status = %resp.status(), "Payment API returned error");

        return Err("Payment API returned error".into());
    }

    let body: PaymentResponse = resp.json().await?;
    if body.success {
        Ok(body.transaction_id)
    } else {
        error!("Payment was declined by provider");

        Err("Payment was declined by provider".into())
    }
}
