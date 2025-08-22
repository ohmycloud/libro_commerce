use leptos::{prelude::*, task::spawn_local};

#[component]
pub fn RegisterForm() -> impl IntoView {
    let (r_username, w_username) = signal("".to_string());
    let (r_email, w_email) = signal("".to_string());
    let (r_message, w_message) = signal("".to_string());
    let on_submit = move |_| {
        let u = r_username.get().clone();
        let e = r_email.get().clone();

        spawn_local(async move {
            let resp: String = reqwest::Client::new()
                .post(format!("/api/register/{}/{}", u, e))
                .send()
                .await
                .unwrap()
                .text()
                .await
                .unwrap();
            w_message.set(resp);
        });
    };

    view! {cx,
        <form on:submit=on_submit>
            <input type="text" placeholder="Username" on:input=move |ev| w_username.set(event_target_value(&ev)) />
            <input type="email" placeholder="Email" on:input=move |ev| w_email.set(event_target_value(&ev)) />
            <button type="submit">Register</button>
        </form>
        <p>
            {r_message.get()}
        </p>
    }
}
