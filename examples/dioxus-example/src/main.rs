#![allow(non_snake_case)]
use dioxus::prelude::*;

mod counter_component;
mod hello_dioxus;

fn main() {
    dioxus::launch(App);
}

fn App() -> Element {
    rsx! {
        hello_dioxus::HelloDioxus {}
        counter_component::CounterComponent {}
    }
}
