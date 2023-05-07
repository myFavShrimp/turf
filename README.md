```scss
// 1. Create a nice style sheet for your component
// This example file is located at `src/HelloWorld.scss`

h1 {
    color: blue;
}

h2 {
    color: red;
}
```

```rust
use leptos::*;

// 2. Use the `style_sheet!` macro and provide the path to your scss file
leptos_grass::style_sheet!("src/HelloWorld.scss");

#[component]
pub fn HelloWorld(cx: Scope, initial_value: i32) -> impl IntoView {

    view! { cx,
        // 3. Create a `style` tag with `STYLE_SHEET` as content
        <style>{STYLE_SHEET}</style>

        // 4. Create a wrapper for your component's content and set the class to `CLASS_NAME`
        <div class=CLASS_NAME>
            <h1>"Hello, World!"</h1>
            <h2>"Hello, Leptos!"</h2>
        </div>
        // 5. Enjoy your blue "Hello, World!" and red "Hello, Leptos!"
    }
}
```
