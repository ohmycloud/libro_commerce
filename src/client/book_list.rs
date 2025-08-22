use leptos::{prelude::*, task::spawn_local};

#[component]
pub fn BookList() -> impl IntoView {
    let (r_books, w_books) = signal(Vec::<String>::new());

    spawn_local(async move {
        let fetched: Vec<String> = reqwest::get("/api/books")
            .await
            .unwrap()
            .json()
            .await
            .unwrap();
        w_books.set(fetched);
    });

    view! {
        <ul>
          {r_books.get().iter().map(|b| view! { cx, <li>{b.clone()}</li>}).collect::<Vec<_>>()}
        </ul>
    }
}
