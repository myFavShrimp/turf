use dioxus::prelude::*;

turf::style_sheet!("src/counter_component.scss");

pub fn CounterComponent(cx: Scope) -> Element {
    let counter = use_state(cx, || 0);

    render! {
        style { STYLE_SHEET }
        div {
            class: ClassName::COUNTER_BUTTON,
            button {
                onclick: move |_| *counter.make_mut() += 1,
                "Click me: {counter}",
            }
        }
    }
}
