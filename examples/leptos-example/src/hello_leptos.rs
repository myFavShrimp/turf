use leptos::*;

turf::configured_style_sheet!("src/hello_leptos.scss");

#[component]
pub fn HelloLeptos(cx: Scope) -> impl IntoView {
    view! { cx,
        <style>{STYLE_SHEET}</style>
        <div class=CLASS_NAME>
            <h1>"Hello, Leptos!"</h1>
        </div>
    }
}
