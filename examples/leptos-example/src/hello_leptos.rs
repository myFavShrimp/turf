use leptos::*;

turf::style_sheet!("src/hello_leptos.scss");

#[component]
pub fn HelloLeptos(cx: Scope) -> impl IntoView {
    view! { cx,
        <style>{STYLE_SHEET}</style>
        <div class=ClassName::HELLO_LEPTOS>
            <h1>"Hello, Leptos!"</h1>
            <h2 class=ClassName::HELLO_WORLD>"Hello, World!"</h2>
        </div>
    }
}
