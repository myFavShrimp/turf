test-lib:
	cargo test --verbose
	cd tests && cargo test --verbose

test-build-examples:
	cd examples/leptos-example && trunk build
	cd examples/leptos-example && trunk build --release
	cd examples/yew-example && trunk build
	cd examples/yew-example && trunk build --release
	cd examples/dioxus-example && trunk build
	cd examples/dioxus-example && trunk build --release
	cd examples/axum-askama-htmx && cargo build
	cd examples/axum-askama-htmx && cargo build --release

test: test-lib test-build-examples
