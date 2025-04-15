use leptos::prelude::*;

mod counter_component;
mod hello_leptos;

use counter_component::*;
use hello_leptos::*;

fn main() {
    mount_to_body(|| {
        view! {
            <HelloLeptos />
            <CounterComponent />
        }
    })
}
