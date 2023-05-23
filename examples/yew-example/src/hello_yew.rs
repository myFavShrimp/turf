use yew::prelude::*;

turf::style_sheet!("src/hello_yew.scss");

#[function_component]
pub fn HelloYew() -> Html {
    html! {
        <div class={CLASS_NAME}>
            <style>{STYLE_SHEET}</style>
            <h1>{"Hello, Yew!"}</h1>
        </div>
    }
}
