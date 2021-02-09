<!-- START doctoc generated TOC please keep comment here to allow auto update -->
<!-- DON'T EDIT THIS SECTION, INSTEAD RE-RUN doctoc TO UPDATE -->
**Table of Contents**  *generated with [DocToc](https://github.com/thlorenz/doctoc)*

- [WASM for NodeJS Sample Application](#wasm-for-nodejs-sample-application)
  - [Command Lines](#command-lines)

<!-- END doctoc generated TOC please keep comment here to allow auto update -->


# WASM for NodeJS Sample Application


* following the guide at https://developer.mozilla.org/en-US/docs/WebAssembly/Rust_to_wasm


## Command Lines

```sh
cargo new --lib hello-wasm && cd hello-wasm
wasm-pack build --target nodejs && trash pkg/.gitignore
```



