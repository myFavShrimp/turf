use yew::prelude::*;

mod counter_component;
mod hello_yew;

use counter_component::*;
use hello_yew::*;

#[function_component]
fn App() -> Html {
    html! {
        <>
            <HelloYew />
            <CounterComponent />
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
