test-lib:
	cd lib && cargo test --verbose
	cd tests && cargo test --verbose

test-lib-once_cell:
	cd lib && cargo test --verbose --features once_cell
	cd tests && cargo test --verbose --features once_cell

test-build-examples:
	cd examples/leptos-example && trunk build
	cd examples/leptos-example && trunk build --release
	cd examples/yew-example && trunk build
	cd examples/yew-example && trunk build --release
	cd examples/dioxus-example && trunk build
	cd examples/dioxus-example && trunk build --release

test: test-lib test-build-examples
