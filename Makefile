binname="wwwapp"


help:
	@echo 'there is no help.. yet'

run: release size
	@cargo run --release

edit:
	@nvim ./src/root_app.rs

edit_main:
	@nvim ./src/main.rs

savetogit:
	@git add . && git commit -m 'saving' && git push

release:
	@cargo rustc --release -- -C prefer-dynamic

cargo-web-release:
	@cargo build --release --target wasm32-unknown-unknown
trunk-release:
	@trunk build --release

test:
	@cargo test

size:
	@ls -lAh ./target/release/$(binname)

clean:
	@cargo clean
