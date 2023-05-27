use yew::prelude::*;

turf::style_sheet!("src/counter_component.scss");

#[function_component]
pub fn CounterComponent() -> Html {
    let state = use_state(|| 0);

    let incr_counter = {
        let state = state.clone();
        Callback::from(move |_| state.set(*state + 1))
    };

    html! {
        <div class={ClassName::COUNTER_BUTTON}>
            <style>{STYLE_SHEET}</style>
            <button onclick={incr_counter}>{"Click me: "} {*state} </button>
        </div>
    }
}
