use leptos::{prelude::*, task::spawn_local};

#[component]
pub fn OrderButton(user_id: u32, book_id: u32) -> impl IntoView {
    let (r_status, w_status) = signal("".to_string());
    let on_click = move |_| {
        spawn_local(async move {
            let resp: String = reqwest::Client::new()
                .post(format!("/api/order/{}/{}", user_id, book_id))
                .send()
                .await
                .unwrap()
                .text()
                .await
                .unwrap();
            w_status.set(resp);
        })
    };

    view! {
        <div>
            <button on:click=on_click>
                Add to Cart
            </button>
            <p>{r_status.get()}</p>
        </div>
    }
}
