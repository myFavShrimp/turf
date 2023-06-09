use yew::prelude::*;

turf::style_sheet!("src/hello_yew.scss");

#[function_component]
pub fn HelloYew() -> Html {
    html! {
        <div class={ClassName::HELLO_YEW}>
            <style>{STYLE_SHEET}</style>
            <h1>{"Hello, Yew!"}</h1>
            <h2 class={ClassName::HELLO_WORLD}>{"Hello, World!"}</h2>
        </div>
    }
}
