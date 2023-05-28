#![allow(non_snake_case)]
use dioxus::prelude::*;

mod counter_component;
mod hello_dioxus;

fn main() {
    dioxus_web::launch(App);
}

fn App(cx: Scope) -> Element {
    cx.render(rsx! {
        hello_dioxus::HelloDioxus {}
        counter_component::CounterComponent {}
    })
}
