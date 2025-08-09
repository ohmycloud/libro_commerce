use leptos::*;

#[component]
pub fn HelloWorld(cx: Scope) -> Element {
    view! {
        cx,
        "Hello, LibroCommerce!",
        "Welcome to our first interactive Leptos interface."
    }
}
