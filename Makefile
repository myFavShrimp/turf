test-release:
	cd lib && cargo test --release --verbose
	cd tests && cargo test --release --verbose

test-debug:
	cd lib && cargo test --verbose
	cd tests && cargo test --verbose

test: test-debug test-release
