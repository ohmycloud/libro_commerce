use leptos::*;

mod hello;

#[wsam_bindgen(start)]
pub fn start() {
    // This will mount our component to the element.
    mount_to_body(|cx| {
        view! {
            cx,
            <hello::HelloWorld/>
        }
    });
}
