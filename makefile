test: test-docs test-base test-all

test-docs:
	cargo test --features downcasting,upcasting,bytearrays --doc

test-base:
	cargo test --lib

test-all:
	cargo test --features 16bit --lib
	cargo test --features 32bit --lib
	cargo test --features 64bit --lib

pre-publish: test
	cargo sync-readme
	cargo deadlinks