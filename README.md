

# WASM for NodeJS Sample Application

<!-- START doctoc generated TOC please keep comment here to allow auto update -->
<!-- DON'T EDIT THIS SECTION, INSTEAD RE-RUN doctoc TO UPDATE -->
**Table of Contents**  *generated with [DocToc](https://github.com/thlorenz/doctoc)*

- [Introductory Notes](#introductory-notes)
- [What it Does](#what-it-does)
- [Samples](#samples)
- [What it Is](#what-it-is)
- [How Does it Compare](#how-does-it-compare)
- [Caveats](#caveats)
- [Command Lines](#command-lines)
- [API](#api)
  - [1.) Persistent State](#1-persistent-state)
  - [2.) Text Preparation](#2-text-preparation)
  - [3.) Text Shaping](#3-text-shaping)
  - [4.) Text Rendering](#4-text-rendering)
  - [5.) Line Breaking](#5-line-breaking)
- [To Do](#to-do)
- [Rendering](#rendering)
- [Also See](#also-see)
  - [Text Shaping](#text-shaping)
  - [Line Breaking](#line-breaking)

<!-- END doctoc generated TOC please keep comment here to allow auto update -->

## Introductory Notes

* this code under development
* this README still fragmentary
* initial goal was to make text shaping as implemented by
  [`rustybuzz`](https://github.com/RazrFalcon/rustybuzz) accessible from NodeJS
* turns out `rustybuzz` depends on [`ttf-parser`](https://docs.rs/ttf-parser/0.11.0/ttf_parser/) which
  offers access to glyf outlines, which is when text rendering (in SVG format) was added
* then I found [`textwrap`](https://docs.rs/textwrap/0.13.2/textwrap/) which implements optimized
  distribution of sized 'black boxes' (i.e. rectangular areas of arbitrary content) over stretches of equal
  length (i.e. lines of text); this solves the problem of wrapping text provided one knows where line break
  opportunities are (e.g. around whitespace, after a hard or soft hyphen, between CJK ideographs)
* add text preparation and you have almost full typesetting (sans combination of styles so far, but we're
  getting there):
  * first do hyphenation for each paragraph of text, which takes some text and some language settings and
    returns the same text with [soft (a.k.a. discretionary, optional) hyphens (`U+00AD` Soft
    Hyphen)](https://en.wikipedia.org/wiki/Soft_hyphen) inserted at appropriate positions;
  * next, apply [*Unicode UAX#14: Unicode Line Breaking Algorithm*](https://www.unicode.org/reports/tr14/)
    to the text; this will identify all the stretches of text that must be kept together in typesetting
    (here dubbed ['slabs'](https://github.com/loveencounterflow/intertext/blob/master/README-slabs.md),
    short for 'syllables')

As it stands, this work will probably be incorporated into
[InterText](https://github.com/loveencounterflow/intertext); at any rate, seeing as the scope of the present
module has been growing, `rustybuzz-wasm` is no longer a fully appropriate moniker.

**TO BE DONE**—For some details around code compilation and installation of this software see [the
installation guide](./INSTALL.md).

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
correct (image of a) text. You can see all this in action in the [live HarfBuzz demo
page](https://harfbuzz.github.io/harfbuzzjs/). If you want to know more about text shaping, be sure to read
[Ramsey Nasser's *Unplain text: A primer on text shaping and rendering non-Latin text in the shadow of an
ASCII-dominated world*](https://increment.com/programming-languages/unplain-text-primer-on-non-latin/);
also, you might want to take a look at the [HarfBuzz terminology
glossary](https://harfbuzz.github.io/terminology.html).


The leading free software to provide text shaping is [HarfBuzz](https://harfbuzz.github.io/) ([repo
here](https://github.com/harfbuzz/harfbuzz)), which is written in C++.
[`rustybuzz`](https://github.com/RazrFalcon/rustybuzz) is "is a complete harfbuzz's shaping algorithm port
to Rust", and since it's written in Rust, we can compile it to WASM and write a nice API surface for it,
which is what I did.


## Samples

![An Arabic Sample](artwork/sample-amiri.png)

*Sample in Arabic, using the [Amiri Typeface](https://www.amirifont.org/) to typeset* "الخط الأمیری".
*Notice visible overlaps and tasteful placement of complex ligatures (which will for the most part not be
present in the browser rendering of the same text unless you happen to configured a suitable font). Both
texts generated from the exact same sequence of Unicode codepoints,* `ا`, `ل`, `خ`, `ط`, `␣`, `ا`, `ل`, `أ`,
`م`, `ی` `ر`, `ی` *(which starts with `ا` and ends with `ی`, notice RTL re-ordering by the browser). Also
note that while the bounding boxes of the glyfs differ in their vertical placements, in this case that only
reflects tthe different areas covered by the outlines; in the underlying SVG, the `y` attributes of all
paths are set to `0` (i.e. all glyfs are still nominally sitting on the baseline).*

![A Tibetan Sample](artwork/sample-tibetan.png)

*Sample in Tibetan, using the [Tibetan Machine Uni
Typeface](https://collab.its.virginia.edu/access/wiki/site/26a34146-33a6-48ce-001e-f16ce7908a6a/tibetan%20machine%20uni.html),
to typeset* ཨོཾ་མ་ཎི་པདྨེ་ཧཱུྃ *(there is a certain chance even in 2021 that this piece of text will not be
rendered correctly across systems and browsers). Again, a complex composition is made from a linear string
of codepoints* `ཨ`, ` ོ`, ` ཾ`, ` ་`, ` མ`, ` ་`, ` ཎ`, ` ི`, ` ་`, ` པ`, ` ད`, ` ྨ`, ` ེ`, ` ་`, ` ཧ`, ` ཱ`,
` ུ`. *Notice that in this font, a choice has been made to precompose the stacked clusters* ` ད`, ` ྨ` *and*
` ཧ`, ` ཱ`, ` ུ`*; this is a design choice which, were it not for a text shaper like `rustybuzz`, would
cause a considerable amount of work for anyone striving to display Tibetan script correctly with this font
and others whose choice of ligatures may be completely different.*



## What it Is

To implement `rustybuzz-wasm` I started with [the example shipped with
`rustybuzz`](https://github.com/RazrFalcon/rustybuzz/blob/master/examples/shape.rs) which compiles to an
executable that accepts a path to a font file and a text and then echoes a containing glyf IDs and
positioning data. This I turned into [a minimalist version with WASM entry
points](https://github.com/loveencounterflow/rustybuzz-wasm/blob/master/src/lib.rs). There's still a lot
missing, especially font feature selection, but since everything went so well so far, I guess I'll get to
that later.

## How Does it Compare

* `rustybuzz-wasm` is not feature-complete with `rustybuzz`, yet.
* `rustybuzz-wasm` would appear to be 1.5 times faster than
  [`harfbuzzjs`](https://github.com/harfbuzz/harfbuzzjs) (which is what drives the [HarfBuzz demo
  page](https://harfbuzz.github.io/harfbuzzjs/)]). `harfbuzzjs` does not allow arbitrarily long lines and
  does not support font features (which `rustybuzz` will probably soon have).
* `rustybuzz-wasm` is over 3 times faster than using
  [`opentype.js`](https://github.com/opentypejs/opentype.js).
* HarfBuzz does have command line utilities, too (referred to as `harfbuzzjs_shaping` in the below benchmark
  results), but the fact that one has to open a sub-process for each piece of text and re-read font files
  damages performance a great deal. This means that `rustybuzz-wasm` (running as WASM attached to a NodeJS
  process) is over 12 times as performant as `harfbuzz` (using child processes over the command line). Note
  that this *does not tell you how fast HarfBuzz itself is* because secondary effects (overhead of one
  process per line of text, re-reading fonts) can be reasonably expected to dominate performance.

The benchmarks ([source
here](https://github.com/loveencounterflow/hengist/blob/master/dev/glyphshapes-and-typesetting-with-harfbuzz/src/textshaping.benchmarks.coffee))
were done with 100 lines of text with 100 words on each line; counts represent Unicode code units (thus,
approximately characters). "1,000 nspc" means "one thousand nanoseconds per cycle", a cycle being the unit
of counting (roughly, one Unicode codepoint); here, lower figurs are better. The reciprocal value expressed
in Hertz (cycles per send) tells you how many items you can expect to get through your chosen process, so
higher numbers are better. The bar charts express relative performance with the top performer being pegged
to 100%. Several runs were performed with randomized order of execution to minimize noise. The hardware is a
2015 customer grade, not fast, not new, not fancy laptop, so many machines will be considerably faster for
all contestants.

```
rustybuzz_wasm_rusty_shaping   0.300 s   65,732 items   218,840⏶Hz     4,570⏷nspc
rustybuzz_wasm_json_shaping    0.368 s   65,732 items   178,605⏶Hz     5,599⏷nspc
rustybuzz_wasm_short_shaping   0.331 s   65,732 items   198,465⏶Hz     5,039⏷nspc
harfbuzzjs_shaping             0.373 s   65,732 items   176,392⏶Hz     5,669⏷nspc
opentypejs_shaping             0.928 s   65,732 items    70,815⏶Hz    14,121⏷nspc
fontkit_shaping                2.203 s   65,732 items    29,840⏶Hz    33,512⏷nspc
harfbuzz_shaping               3.745 s   65,732 items    17,553⏶Hz    56,971⏷nspc

rustybuzz_wasm_rusty_shaping     220,399 Hz   100.0 % │████████████▌│
rustybuzz_wasm_short_shaping     194,886 Hz    88.4 % │███████████  │
rustybuzz_wasm_json_shaping      180,277 Hz    81.8 % │██████████▎  │
harfbuzzjs_shaping               143,434 Hz    65.1 % │████████▏    │
opentypejs_shaping                65,468 Hz    29.7 % │███▊         │
fontkit_shaping                   29,605 Hz    13.4 % │█▋           │
harfbuzz_shaping                  17,153 Hz     7.8 % │█            │
```

## Caveats

* Rust Newbie here so probably the code is not ideal in some respects.
* FTTB I have commited the WASM artefacts to the repo; since I'm still working on this you may happen to
  download some **unoptimized code which is orders of magnitude slower than WASM resulting from optimized
  compilation**
* Always re-build before trying out:
  * for faster compilation, do `wasm-pack build --debug --target nodejs && trash pkg/.gitignore && node
    demo-nodejs-using-wasm/lib/main.js > /tmp/foo.svg`
  * for faster execution, do `wasm-pack build         --target nodejs && trash pkg/.gitignore && node
    demo-nodejs-using-wasm/lib/main.js > /tmp/foo.svg`
* Values are currently communicated as JSON and hex-encoded binary strings; this is probably not terribly
  efficient and may change in the future; see https://hacks.mozilla.org/2019/11/multi-value-all-the-wasm/
  and https://docs.rs/serde-wasm-bindgen/0.1.3/serde_wasm_bindgen/.

## Command Lines

To build and test in dev (much faster, but also *much* slower)

```sh
wasm-pack build --debug --target nodejs && trash pkg/.gitignore && ~/jzr/nodexh/bin/nodexh ~/temp/hello-wasm/demo-nodejs-using-wasm/lib/main.js
```

To build and test production:

```sh
wasm-pack build --target nodejs && trash pkg/.gitignore && ~/jzr/nodexh/bin/nodexh ~/temp/hello-wasm/demo-nodejs-using-wasm/lib/main.js
```

## API

### 1.) Persistent State

* **`pub fn set_font_bytes( font_bytes_hex: String ) {`**—
* **`pub fn has_font_bytes() -> bool { unsafe { !FONT_BYTES.is_empty() } }`**—

### 2.) Text Preparation

### 3.) Text Shaping

* **`pub fn shape_text( user_cfg: &JsValue ) -> String {`**—
<!-- * **`pub fn glyfs_as_json( glyph_buffer: &rustybuzz::GlyphBuffer, ) -> String {`**— -->
<!-- * **`pub fn glyfs_as_short( glyph_buffer: &rustybuzz::GlyphBuffer, ) -> String {`**— -->

### 4.) Text Rendering

* **`pub fn glyph_to_svg_pathdata( js_glyph_id: &JsValue ) -> String {`**—

### 5.) Line Breaking

* **`pub fn wrap_text( text: String, width: usize ) -> String {`**—

## To Do

* [ ] find out what makes format `rusty` (which has quite a few options) so much faster than the
  minimalistic `short` format (which has no options); to do so, modify the (constant) format flags
* [ ] implement OpenType font features
* [ ] implement face selection
* [ ] implement language selection?
* [ ] implement script selection?
* [ ] implement clustering selection?

## Rendering

* [`ab-glyph`](https://github.com/alexheretic/ab-glyph)—"When laying out glyphs into paragraph, ab_glyph is
  faster than rusttype using .ttf fonts & much faster for .otf fonts."
* [`rusttype`](https://gitlab.redox-os.org/redox-os/rusttype)—A pure Rust alternative to libraries like
  FreeType
* [Fontdue](https://github.com/mooman219/fontdue)—Fontdue is a simple, `no_std` (does not use the standard
  library for portability), pure Rust, TrueType (`.ttf/.ttc`) & OpenType (`.otf`) font rasterizer and layout
  tool. It strives to make interacting with fonts as fast as possible, and currently has the lowest end to
  end latency for a font rasterizer.

## Also See

### Text Shaping

* [Allsorts](https://github.com/yeslogic/allsorts)—Allsorts is a font parser, shaping engine, and subsetter
  for OpenType, WOFF, and WOFF2 written entirely in Rust. It was extracted from
  [Prince](https://www.princexml.com/), a tool that typesets and lays out HTML and CSS documents into PDF.

  The Allsorts shaping engine was developed in conjunction with [a specification for OpenType
  shaping](https://github.com/n8willis/opentype-shaping-documents/), which aims to specify OpenType font
  shaping behaviour.

* [OpenType shaping documents](https://github.com/n8willis/opentype-shaping-documents/)

### Line Breaking

* [newbreak](https://github.com/simoncozens/newbreak)—written in JS/TS, has tentative Rust implementation;
  TS fails to compile to JS; last commits in Summer 2020 so maybe abandoned.
* [fontdue](https://github.com/mooman219/fontdue)—written in Rust, aims to be font rasterizer *including*
  text wrapping, but sadly [fails to compile although I could hotfix
  that](https://github.com/mooman219/fontdue/issues/57).
* Repeating links from [a StackOverflow answer](https://cs.stackexchange.com/a/123303):
* [kas-text](https://github.com/kas-gui/kas-text) looks enticing but is a huge thing geared towards building
  GUI apps. It uses the original HarfBuzz C libraries so I rather not touch this thing as C dependencies
  will always be cans of worms.

