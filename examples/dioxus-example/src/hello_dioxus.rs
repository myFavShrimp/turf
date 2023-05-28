use dioxus::prelude::*;

turf::style_sheet!("src/hello_dioxus.scss");

pub fn HelloDioxus(cx: Scope) -> Element {
    render! {
        style { STYLE_SHEET }
        div {
            class: ClassName::HELLO_DIOXUS,
            h1 { "Hello, Dioxus!" }
            h2 {
                class: ClassName::HELLO_WORLD,
                "Hello, World!"
            }
        }
    }
}
