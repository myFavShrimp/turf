use dioxus::prelude::*;

turf::style_sheet!("src/counter_component.scss");

pub fn CounterComponent() -> Element {
    let mut counter = use_signal(|| 0);

    rsx! {
        style { "{STYLE_SHEET}" }
        div {
            class: ClassName::COUNTER_BUTTON,
            button {
                onclick: move |_| *counter.write() += 1,
                "Click me: {counter}",
            }
        }
    }
}
