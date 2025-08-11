use leptos::*;

#[component]
pub fn RegisterForm(cx: Scope) -> Element {
    let username = create_signal(cx, "".to_string());
    let email = create_signal(cx, "".to_string());
    let message = create_signal(cx, "".to_string());
    let on_submit = move |_| {
        let u = username.get().clone();
        let e = email.get().clone();

        spawn_local(async move {
            let resp: String = reqwest::Client::new()
                .post(format!("/api/register/{}/{}", u, e))
                .send()
                .await
                .unwrap()
                .text()
                .await
                .unwrap();
            message.set(resp);
        });
    };

    view! {cx,
        <form on:submit=on_submit>
            <input type="text" placeholder="Username" on:input=move |ev| username.set(event_target_value(&ev)) />
            <input type="email" placeholder="Email" on:input=move |ev| email.set(event_target_value(&ev)) />
            <button type="submit">Register</button>
        </form>
        <p>
            {message.get()}
        </p>
    }
}
