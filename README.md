

# WASM for NodeJS Sample Application

<!-- START doctoc generated TOC please keep comment here to allow auto update -->
<!-- DON'T EDIT THIS SECTION, INSTEAD RE-RUN doctoc TO UPDATE -->
**Table of Contents**  *generated with [DocToc](https://github.com/thlorenz/doctoc)*

- [Steps to Follow](#steps-to-follow)
- [Installation](#installation)
- [Command Lines](#command-lines)

<!-- END doctoc generated TOC please keep comment here to allow auto update -->

## Steps to Follow

* following the guide at https://developer.mozilla.org/en-US/docs/WebAssembly/Rust_to_wasm

## Installation

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


## Command Lines

```sh
cargo new --lib hello-wasm && cd hello-wasm
wasm-pack build --target nodejs && trash pkg/.gitignore
```



