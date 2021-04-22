test: test-docs test-base test-all

test-docs:
	cargo test --features downcasting,upcasting,bytearrays --doc

test-base:
	cargo test --lib

test-all:
	cargo test --features 16bit --lib
	cargo test --features 32bit --lib
	cargo test --features 64bit --lib

benchmark:
	cargo bench --features upcasting,downcasting,bytearrays
	cargo bench --features 16bit,upcasting,downcasting,bytearrays
	cargo bench --features 32bit,upcasting,downcasting,bytearrays
	cargo bench --features 64bit,upcasting,downcasting,bytearrays

pre-publish: test benchmark
	cargo sync-readme
	cargo deadlinks