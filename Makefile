binname="wwwapp"


help:
	@echo 'there is no help.. yet'

edit:
	@nvim ./src/root_app.rs
edit_main:
	@nvim ./src/main.rs

savetogit:
	@git add . && git commit -m 'saving' && git push

release:
	@cargo rustc --release -- -C prefer-dynamic
run: release size
	@cargo run --release

trunk-release:
	@trunk build --release


all: release trunk-release savetogit


test:
	@cargo test

size:
	@ls -lAh ./target/release/$(binname)

clean:
	@cargo clean
