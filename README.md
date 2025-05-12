# WASM is fun!

This is a simple playground with [WebAssembly (WASM)](https://webassembly.org) and Rust. It allows you to compile Rust code to WebAssembly and run it in the browser.

## Requirements

- [Rust](https://www.rust-lang.org/tools/install)
- [wasm-bindgen](https://rustwasm.github.io/docs/wasm-bindgen/)
- [make](https://www.gnu.org/software/make/)

You can run the project simply by running `make` to compile the Rust code to WebAssembly and then serve the files using your preferred web server!

```bash
make

npx vite
```

## Why?

I wanted to better understand how WASM works and it can be used in the web. And I was bored :)
