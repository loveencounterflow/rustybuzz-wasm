

# WASM for NodeJS Sample Application

<!-- START doctoc generated TOC please keep comment here to allow auto update -->
<!-- DON'T EDIT THIS SECTION, INSTEAD RE-RUN doctoc TO UPDATE -->
**Table of Contents**  *generated with [DocToc](https://github.com/thlorenz/doctoc)*

- [What it Does](#what-it-does)
- [What it Is](#what-it-is)
- [Steps to Follow](#steps-to-follow)
- [Installation](#installation)
- [Publish Compiled WASM Code](#publish-compiled-wasm-code)
- [Command Lines](#command-lines)
- [Benchmarks](#benchmarks)
- [To Do](#to-do)

<!-- END doctoc generated TOC please keep comment here to allow auto update -->

## What it Does

This module allows users to take a Unicode text and a path to a font file as inputs and obtain a list of
GlyfIDs and 2D positions back. This process is known as [text
shaping](https://en.wikipedia.org/wiki/Complex_text_layout). It is an indespensible ingredient for
compositing text in so-called 'complex' writing systems like Arabic and Indic alphabets, but even when
applied to text written in the Latin alphabet, there are finer points of typesetting like
[kerning](https://en.wikipedia.org/wiki/Kerning) and the choice of
[ligatures](https://en.wikipedia.org/wiki/Orthographic_ligature) which makes this process too difficult to
be reasonably implemented on-the-fly for each piece of software that uses text. Instead, what one wants is a
specialized library that knows lots of details about font file formats, OpenType font features, type metrics
and so on and applies that knowledge to a given text string to derive poisitioning data for the individual
graphical pieces ('glyfs') that, when drawn out on a canvas (such as an HTML `<canvas>` or an `<svg>`
element) then instruct the rendering software to render an aesthetically pleasing and orthographically
correct (image of a) text.


## What it Is

To implement `rustybuzz-wasm` I started with [the example shipped with
`rustybuzz`](https://github.com/RazrFalcon/rustybuzz/blob/master/examples/shape.rs) which compiles to an
executable that accepts a path to a font file and a text and then echoes a containing glyf IDs and
positioning data.

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

## Publish Compiled WASM Code

```sh
pnpm version patch && pnpm publish --access public && git push
```

## Command Lines

```sh
cargo new --lib hello-wasm && cd hello-wasm
wasm-pack build --target nodejs && trash pkg/.gitignore
```

To build and test in dev (much faster, but also *much* slower)

```sh
wasm-pack build --debug --target nodejs && trash pkg/.gitignore && ~/jzr/nodexh/bin/nodexh ~/temp/hello-wasm/demo-nodejs-using-wasm/lib/main.js
```

To build and test production:

```sh
wasm-pack build --target nodejs && trash pkg/.gitignore && ~/jzr/nodexh/bin/nodexh ~/temp/hello-wasm/demo-nodejs-using-wasm/lib/main.js
```

## Benchmarks

Yay!

```
00:14 TEXTSHAPING  ▶  ------------------------------------------------------------------------------------------------------------
opentypejs_shaping                         0.928 s          65,732 items          70,815⏶Hz          14,121⏷nspc
harfbuzzjs_shaping                         0.373 s          65,732 items         176,392⏶Hz           5,669⏷nspc
rustybuzz_wasm_short_shaping               0.331 s          65,732 items         198,465⏶Hz           5,039⏷nspc
harfbuzz_shaping                           3.745 s          65,732 items          17,553⏶Hz          56,971⏷nspc
rustybuzz_wasm_json_shaping                0.368 s          65,732 items         178,605⏶Hz           5,599⏷nspc
rustybuzz_wasm_rusty_shaping               0.300 s          65,732 items         218,840⏶Hz           4,570⏷nspc
fontkit_shaping                            2.203 s          65,732 items          29,840⏶Hz          33,512⏷nspc
00:23 HENGIST/BENCHMARKS  ▶  rustybuzz_wasm_rusty_shaping                     220,399 Hz   100.0 % │████████████▌│
00:23 HENGIST/BENCHMARKS  ▶  rustybuzz_wasm_short_shaping                     194,886 Hz    88.4 % │███████████  │
00:23 HENGIST/BENCHMARKS  ▶  rustybuzz_wasm_json_shaping                      180,277 Hz    81.8 % │██████████▎  │
00:23 HENGIST/BENCHMARKS  ▶  harfbuzzjs_shaping                               143,434 Hz    65.1 % │████████▏    │
00:23 HENGIST/BENCHMARKS  ▶  opentypejs_shaping                                65,468 Hz    29.7 % │███▊         │
00:23 HENGIST/BENCHMARKS  ▶  fontkit_shaping                                   29,605 Hz    13.4 % │█▋           │
00:23 HENGIST/BENCHMARKS  ▶  harfbuzz_shaping                                  17,153 Hz     7.8 % │█            │
```

## To Do

* [ ] find out what makes format `rusty` (which has quite a few options) so much faster than the
  minimalistic `short` format (which has no options); to do so, modify the (constant) format flags


