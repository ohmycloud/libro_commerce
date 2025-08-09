use leptos::*;

#[component]
pub fn HelloWorld(cx: Scope) -> Element {
    view! {
        cx,
        <div>
            <h1>"Hello, LibroCommerce!"</h1>
            <p>"Welcome to our first interactive Leptos interface."</p>
        </div>
    }
}
