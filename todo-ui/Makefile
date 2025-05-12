TODO_UI=todo_ui

all:
	@cargo build --target wasm32-unknown-unknown --release
	@wasm-bindgen target/wasm32-unknown-unknown/release/$(TODO_UI).wasm  --out-dir ./pkg --target web
	@echo "[\033[1;32mOK\033[0m] Build complete"

clean:
	@cargo clean
	@rm -rf pkg
	@echo "[\033[1;32mOK\033[0m] Clean complete"

re: clean all
