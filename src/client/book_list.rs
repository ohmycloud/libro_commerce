use leptos::*;

#[component]
pub fn BookList(cx: Scope) -> Element {
    let books = create_signal(cx, Vec::<String>::new());

    spawn_local(async move {
        let fetched: Vec<String> = reqwest::get("/api/books")
            .await
            .unwrap()
            .json()
            .await
            .unwrap();
        books.set(fetched);
    });

    view! {cx,
        <ul>
          {books.get().iter().map(|b| view! { cx, <li>{b.clone()}</li>}).collect::<Vec<_>>()}
        </ul>
    }
}
