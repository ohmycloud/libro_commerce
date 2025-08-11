use leptos::*;

#[component]
pub fn OrderButton(cx: Scope, user_id: u32, book_id: u32) -> Element {
    let status = create_signal(cx, "".to_string());
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
            status.set(resp);
        })
    };

    view! {cx,
        <div>
            <button on:click=on_click>
                Add to Cart
            </button>
            <p>{status.get()}</p>
        </div>
    }
}
