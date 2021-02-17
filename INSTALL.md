
## Steps to Follow

* following the guide at https://developer.mozilla.org/en-US/docs/WebAssembly/Rust_to_wasm

## Installation


<strike>```` * create project ````</strike><br>

<strike>```` ```sh ````</strike><br>
<strike>```` cargo new --lib hello-wasm && cd hello-wasm ````</strike><br>
<strike>```` ``` ````</strike><br>

<strike>```` * edit `Cargo.toml`: ````</strike><br>

<strike>```` ```toml ````</strike><br>
<strike>```` [lib] ````</strike><br>
<strike>```` crate-type = ["cdylib"] ````</strike><br>

<strike>```` [dependencies] ````</strike><br>
<strike>```` wasm-bindgen = "0.2" ````</strike><br>
<strike>```` pico-args = "0.3" ````</strike><br>
<strike>```` libc = "0.2" ````</strike><br>
<strike>```` ``` ````</strike><br>

<strike>```` * install `wasm-pack`: ````</strike><br>

<strike>```` ```sh ````</strike><br>
<strike>```` cargo install wasm-pack ````</strike><br>
<strike>```` ``` ````</strike><br>

<!--
* create project

```sh
cargo new --lib hello-wasm && cd hello-wasm
```

* edit `Cargo.toml`:

```toml
[lib]
crate-type = ["cdylib"]

[dependencies]
wasm-bindgen = "0.2"
pico-args = "0.3"
libc = "0.2"
```

* install `wasm-pack`:

```sh
cargo install wasm-pack
```
-->

## Publish Compiled WASM Code

```sh
pnpm version patch && pnpm publish --access public && git push
```
