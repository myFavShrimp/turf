test-lib:
	cargo test --verbose --workspace
	cd tests && cargo test --verbose --workspace

test-build-examples:
	cd examples/leptos-example && trunk build
	cd examples/leptos-example && trunk build --release
	cd examples/yew-example && trunk build
	cd examples/yew-example && trunk build --release
	cd examples/dioxus-example && trunk build
	cd examples/dioxus-example && trunk build --release
	cd examples/axum-askama-htmx && cargo build
	cd examples/axum-askama-htmx && cargo build --release
	cd examples/leptos-hash-example && trunk build
	cd examples/leptos-hash-example && trunk build --release

test: test-lib test-build-examples
