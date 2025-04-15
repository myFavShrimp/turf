use leptos::prelude::*;

turf::style_sheet!("src/counter_component.scss");

#[component]
pub fn CounterComponent() -> impl IntoView {
    let (count, set_count) = signal(0);

    view! {
        <style>{STYLE_SHEET}</style>
        <div class=ClassName::COUNTER_BUTTON>
            <button on:click=move |_| { set_count.update(|n| *n += 1) }>
                "Click me: "
                {move || count.get()}
            </button>
        </div>
    }
}
