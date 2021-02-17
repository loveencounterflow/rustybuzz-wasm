#![allow(dead_code)]
#![allow(unused_variables)]

extern crate serde_json;
extern crate wasm_bindgen;
extern crate hex;
use wasm_bindgen::prelude::*;

#[macro_use]
extern crate serde_derive;

#[wasm_bindgen]
extern {
  pub fn info(  s: &str );
  pub fn alert( s: &str );
  pub fn help(  s: &str );
  pub fn urge(  s: &str ); }

//----------------------------------------------------------------------------------------------------------
use std::str::FromStr;
use rustybuzz;
use ttf_parser;
// use svgtypes::WriteBuffer;
use svgtypes::PathSegment;
use serde_json::json;
use textwrap;
// use std::collections::HashMap;
// #[macro_use]
// extern crate lazy_static;


//==========================================================================================================
// PERSISTENT STATE
// //----------------------------------------------------------------------------------------------------------
// // thx to https://stackoverflow.com/a/19608953/256361
// //----------------------------------------------------------------------------------------------------------
// lazy_static! {
//   // thx to https://docs.rs/lazy_static/1.4.0/lazy_static/ and Rust error messages
//   static ref CACHE: Vec<&'static CacheEntry<'static>> = {
//     #![allow(unused_mut)]
//     let mut m = Vec::new();
//     // m.insert(0, "foo");
//     m
// }; }

// thx to https://stackoverflow.com/a/27826181/256361
// use lazy_static::lazy_static; // 1.4.0
// use std::sync::Mutex;

// lazy_static! {
//     static ref CACHE: Mutex<Vec<CacheEntry<'static>>> = Mutex::new(vec![]);
// }

// fn do_a_call() { CACHE.lock().unwrap().push(1);}
// fn main() {
//     do_a_call();
//     do_a_call();
//     do_a_call();
//     println!("called {}", CACHE.lock().unwrap().len());}

// //----------------------------------------------------------------------------------------------------------
// pub struct CacheEntry<'a> {
//   rustybuzz_face:    rustybuzz::Face<'a>,
//   ttfparser_face:   ttf_parser::Face<'a>,
//   // scale: ...,
// }

static mut FONTBYTES_0: Vec<u8> = vec![];
static mut FONTBYTES_1: Vec<u8> = vec![];
static mut FONTBYTES_2: Vec<u8> = vec![];
static mut FONTBYTES_3: Vec<u8> = vec![];


static mut FONT_BYTES: Vec<u8> = vec![];
//----------------------------------------------------------------------------------------------------------
#[wasm_bindgen]
pub fn set_font_bytes( font_bytes_hex: String ) {
  unsafe { FONT_BYTES = match hex::decode( font_bytes_hex ) {
    Ok( v ) => v,
    Err( error ) => {
      alert( &format!( "^895734^ error decoding hexadecimal: {}", error ) );
      std::process::exit( 1 ); }, }; }; }
//----------------------------------------------------------------------------------------------------------
#[wasm_bindgen]
pub fn has_font_bytes() -> bool { unsafe { !FONT_BYTES.is_empty() } }

//----------------------------------------------------------------------------------------------------------
// #![allow(unused_mut)]
#[wasm_bindgen]
pub fn register_font( font_idx: u16, font_bytes_hex: String ) {
  // if font_idx > 4 {
  //   alert( &format!( "^895455^ font_idx must be between 1 and 3, got {}", font_idx ) );
  //   std::process::exit( 1 ); };
  let face_index = 0;
  let font_bytes = match hex::decode( font_bytes_hex ) {
    Ok( v ) => v,
    Err( error ) => {
      alert( &format!( "^895734^ error decoding hexadecimal: {}", error ) );
      std::process::exit( 1 ); }, };
  unsafe {
    match font_idx {
      0 => FONTBYTES_0 = font_bytes,
      1 => FONTBYTES_1 = font_bytes,
      2 => FONTBYTES_2 = font_bytes,
      3 => FONTBYTES_3 = font_bytes,
      4u16..=std::u16::MAX => {
        alert( &format!( "^895433^ font_idx must be between 0 and 3, got {}", font_idx ) );
        std::process::exit( 1 ); }

      }
    }
  // let rustybuzz_face =  rustybuzz::Face::from_slice( &font_bytes, face_index ).unwrap();
  // let     ttfparser_face = ttf_parser::Face::from_slice( &font_bytes, face_index ).unwrap();
  // let entry = CacheEntry {
  //   rustybuzz_face: rustybuzz_face,
  //   ttfparser_face: ttfparser_face,

  // };
  // // CACHE.push( &entry );
  // CACHE.lock().unwrap().push( entry );
  // CACHE.push( 42 );
  // (&mut *HASHMAP.borrow_mut())(); // see https://stackoverflow.com/a/59835163/256361
  // (&mut HASHMAP).insert( name, &entry );
}


//==========================================================================================================
// CONFIGURATION
//----------------------------------------------------------------------------------------------------------
#[derive(Serialize, Deserialize)]
// acc to https://doc.rust-lang.org/std/fmt/trait.Debug.html
#[derive(Debug)]
pub struct CfgOpt {
    pub text:             Option<String>,
    // pub font_bytes_hex:   Option<String>,
    pub format:           Option<String>,
    pub font_ptem:        Option<f32>, }

#[derive(Debug)]
pub struct Cfg {
    pub text:             String,
    pub cluster_level:    rustybuzz::BufferClusterLevel,
    pub direction:        rustybuzz::Direction,
    pub face_index:       u32,
    // pub font_bytes:       Vec<u8>,
    pub font_ptem:        f32,
    pub format:           String,
    pub variations:       Vec<rustybuzz::Variation>,
    pub features:         Vec<rustybuzz::Feature>,
    pub script:           Option<rustybuzz::Script>,
    pub language:         rustybuzz::Language, }

const FONT_SIZE: f64 = 1000.0;
const PRECISION: f64 = 1.0;


//==========================================================================================================
//
//----------------------------------------------------------------------------------------------------------
fn parse_features(s: &str) -> Result<Vec<rustybuzz::Feature>, String> {
    let mut features = Vec::new();
    for f in s.split(',') {
        features.push(rustybuzz::Feature::from_str(&f)?); }
    Ok(features) }


//==========================================================================================================
//
//----------------------------------------------------------------------------------------------------------
// thx to https://hacks.mozilla.org/2019/11/multi-value-all-the-wasm/
#[wasm_bindgen]
pub fn shape_text( user_cfg: &JsValue ) -> String {
  //........................................................................................................
  let cfg_opt: CfgOpt = user_cfg.into_serde().unwrap();
  // //........................................................................................................
  // let font_bytes = match cfg_opt.font_bytes_hex {
  //   None => vec![],
  //   Some( x ) => match hex::decode( x ) {
  //     Ok( v ) => v,
  //     Err( error ) => {
  //       alert( &format!( "^89573485^ error decoding hexadecimal: {}", error ) );
  //       std::process::exit( 1 ); }, }, };
  //........................................................................................................
  let cfg = Cfg {
    text:           match cfg_opt.text { None => String::from( "some text" ), Some( x ) => x, },
    // ### TAINT use enumeration
    format:         match cfg_opt.format { None => String::from( "json" ), Some( x ) => x, },
    // font_bytes:     font_bytes,
    font_ptem:      match cfg_opt.font_ptem { None => 1000.0, Some( x ) => x, },
    language:       rustybuzz::Language::from_str( "English" ).unwrap(),
    //......................................................................................................
    // script:         Some( rustybuzz::Script::new() ),
    script:         None,
    features:       vec![],
    variations:     vec![],
    direction:      rustybuzz::Direction::LeftToRight,
    cluster_level:  rustybuzz::BufferClusterLevel::MonotoneGraphemes,
    // cluster_level: rustybuzz::BufferClusterLevel::MonotoneCharacters,
    // cluster_level: rustybuzz::BufferClusterLevel::Characters,
    face_index:   0, };
  //........................................................................................................
  // ### TAINT use cache for face_index, face
  // ### TAINT almost identical to `ttf_parser::Face`
  let mut face = unsafe { rustybuzz::Face::from_slice(&FONT_BYTES, cfg.face_index).unwrap() };
  // ### TAINT use `set_pixels_per_em()`?
  face.set_points_per_em( Some( cfg.font_ptem ) );
  if !cfg.variations.is_empty() { face.set_variations( &cfg.variations ); }
  let mut buffer = rustybuzz::UnicodeBuffer::new();
  buffer.push_str( &cfg.text );
  buffer.set_direction( cfg.direction );
  //........................................................................................................
  buffer.set_language(cfg.language);
  if let Some(script) = cfg.script { buffer.set_script(script); }
  buffer.set_cluster_level(cfg.cluster_level);
  // if !cfg.utf8_clusters { buffer.reset_clusters(); }
  //........................................................................................................
  let glyph_buffer = rustybuzz::shape( &face, &cfg.features, buffer );
  //........................................................................................................
  if cfg.format == "json" { return glyfs_as_json( &glyph_buffer, ); }
  else if cfg.format == "rusty" {
    let format_flags: rustybuzz::SerializeFlags =
      // rustybuzz::SerializeFlags::NO_GLYPH_NAMES |
      // rustybuzz::SerializeFlags::GLYPH_EXTENTS |
      rustybuzz::SerializeFlags::GLYPH_FLAGS;
    return glyph_buffer.serialize( &face, format_flags ); }
  // urge( &format!( "^33321^ {}", glyfs_as_short( &glyph_buffer, ) ) );
  return glyfs_as_short( &glyph_buffer, ); }


//==========================================================================================================
//
//----------------------------------------------------------------------------------------------------------
pub fn glyfs_as_json( glyph_buffer: &rustybuzz::GlyphBuffer, ) -> String {
  _glyfs_as_json( &glyph_buffer, ).unwrap_or_default() }

//----------------------------------------------------------------------------------------------------------
fn _glyfs_as_json( glyph_buffer: &rustybuzz::GlyphBuffer, ) -> Result<String, std::fmt::Error> {
  use std::fmt::Write;
  let mut s = String::with_capacity(64);
  let info  = glyph_buffer.glyph_infos();
  let pos   = glyph_buffer.glyph_positions();
  let mut x = 0;
  let mut y = 0;
  write!(&mut s, "[" )?;
  for (info, pos) in info.iter().zip(pos) {
    write!(&mut s, "{{" )?;
    write!(&mut s, "\"gid\":{},", info.codepoint)?;
    write!(&mut s, "\"x\":{},\"y\":{},", x, y )?;
    write!(&mut s, "\"dx\":{},\"dy\":{}", pos.x_advance, pos.y_advance )?;
    x += pos.x_advance;
    y += pos.y_advance;
    //....................................................................................................
    write!(&mut s, "}}" )?;
    s.push(','); }
  //........................................................................................................
  if !s.is_empty() { s.pop(); } // Remove last `,`
  write!(&mut s, "]" )?;
  Ok(s) }


//==========================================================================================================
//
//----------------------------------------------------------------------------------------------------------
pub fn glyfs_as_short( glyph_buffer: &rustybuzz::GlyphBuffer, ) -> String {
  _glyfs_as_short( &glyph_buffer, ).unwrap_or_default() }

//----------------------------------------------------------------------------------------------------------
fn _glyfs_as_short( glyph_buffer: &rustybuzz::GlyphBuffer, ) -> Result<String, std::fmt::Error> {
  use std::fmt::Write;
  let mut s = String::with_capacity(64);
  let info  = glyph_buffer.glyph_infos();
  let pos   = glyph_buffer.glyph_positions();
  let mut x = 0;
  let mut y = 0;
  write!(&mut s, "|" )?;
  for (info, pos) in info.iter().zip(pos) {
    write!(&mut s, "{}:", info.codepoint)?;
    write!(&mut s, "{},{};", x, y )?;
    write!(&mut s, "{},{}", pos.x_advance, pos.y_advance )?;
    x += pos.x_advance;
    y += pos.y_advance;
    //....................................................................................................
    write!(&mut s, "|" )?;
    }
  //........................................................................................................
  Ok(s) }


//==========================================================================================================
//
//----------------------------------------------------------------------------------------------------------
// #[wasm_bindgen]
// pub fn glyphs_to_path( glyph_ids: &JsValue ) -> Vec<String> {
//   return String::from( "" ); }

#[wasm_bindgen]
pub fn glyph_to_svg_pathdata( js_glyph_id: &JsValue ) -> String {
  // ### TAINT try to shorten
  let glyph_id_u16: u16 = js_glyph_id.into_serde().unwrap();
  let glyph_id: ttf_parser::GlyphId = ttf_parser::GlyphId( glyph_id_u16 );
  //........................................................................................................
  // ### TAINT use cache for face_index, face
  // ### TAINT almost identical to `rustybuzz::Face`
  let face_index    = 0;
  let face          = unsafe { ttf_parser::Face::from_slice(&FONT_BYTES, face_index).unwrap() };
  let units_per_em  = match face.units_per_em() { None => FONT_SIZE as u16, Some( x ) => x, };
  let scale         = FONT_SIZE / units_per_em as f64;
  let mut path_buf  = svgtypes::Path::with_capacity(64);
  let mut builder   = Builder( &mut path_buf );
  let bbox          = face.outline_glyph( glyph_id, &mut builder );
  for seg in path_buf.iter_mut() { scale_segment( seg, scale ); };
  let bbox_svg     = rectangle_from_bbox( match bbox {
    None      => ttf_parser::Rect { x_min: 0, y_min: 0, x_max: 0, y_max: 0, },
    Some( x ) => x, },
    scale );
  //........................................................................................................
  return json!({
    "pd": path_buf.to_string(),
    "br": bbox_svg,
  }).to_string();
}

//----------------------------------------------------------------------------------------------------------
fn rectangle_from_bbox( bbox: ttf_parser::Rect, scale: f64, ) -> String {
  return format!( "<rect x=\"{}\" y=\"{}\" width=\"{}\" height=\"{}\"/>",
    scale_coordinate(  bbox.x_min    as f64, scale ),
    scale_coordinate(  ( - bbox.y_min - bbox.height() )   as f64, scale ),
    scale_coordinate(  bbox.width()  as f64, scale ),
    scale_coordinate(  bbox.height() as f64, scale ), ) }

//----------------------------------------------------------------------------------------------------------
struct Builder<'a>(&'a mut svgtypes::Path);
  /// see https://docs.rs/ttf-parser/0.11.0/ttf_parser/struct.FaceTables.html#method.outline_glyph

impl ttf_parser::OutlineBuilder for Builder<'_> {
  fn move_to(&mut self, x: f32, y: f32) {
    self.0.push_move_to(x as f64, -y as f64); }

  fn line_to(&mut self, x: f32, y: f32) {
    self.0.push_line_to(x as f64, -y as f64); }

  fn quad_to(&mut self, x1: f32, y1: f32, x: f32, y: f32) {
    self.0.push_quad_to(x1 as f64, -y1 as f64, x as f64, -y as f64); }

  fn curve_to(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, x: f32, y: f32) {
    self.0.push_curve_to(x1 as f64, -y1 as f64, x2 as f64, -y2 as f64, x as f64, -y as f64); }

  fn close(&mut self) {
    self.0.push_close_path(); }
  }

//----------------------------------------------------------------------------------------------------------
fn scale_coordinate( a: f64, scale: f64 ) -> f64 { ( a  * scale * PRECISION ).round() / PRECISION }

//----------------------------------------------------------------------------------------------------------
fn scale_segment(d: &mut PathSegment, scale: f64 ) {
  match *d {
    PathSegment::MoveTo { ref mut x, ref mut y, .. } => {
      *x  = ( *x  * scale * PRECISION ).round() / PRECISION;
      *y  = ( *y  * scale * PRECISION ).round() / PRECISION; }
    PathSegment::LineTo { ref mut x, ref mut y, .. } => {
      *x  = ( *x  * scale * PRECISION ).round() / PRECISION;
      *y  = ( *y  * scale * PRECISION ).round() / PRECISION; }
    PathSegment::HorizontalLineTo { ref mut x, .. } => {
      *x  = ( *x  * scale * PRECISION ).round() / PRECISION; }
    PathSegment::VerticalLineTo { ref mut y, .. } => {
      *y  = ( *y  * scale * PRECISION ).round() / PRECISION; }
    PathSegment::CurveTo { ref mut x1, ref mut y1, ref mut x2, ref mut y2, ref mut x, ref mut y, .. } => {
      *x1 = ( *x1 * scale * PRECISION ).round() / PRECISION;
      *y1 = ( *y1 * scale * PRECISION ).round() / PRECISION;
      *x2 = ( *x2 * scale * PRECISION ).round() / PRECISION;
      *y2 = ( *y2 * scale * PRECISION ).round() / PRECISION;
      *x  = ( *x  * scale * PRECISION ).round() / PRECISION;
      *y  = ( *y  * scale * PRECISION ).round() / PRECISION; }
    PathSegment::SmoothCurveTo { ref mut x2, ref mut y2, ref mut x, ref mut y, .. } => {
      *x2 = ( *x2 * scale * PRECISION ).round() / PRECISION;
      *y2 = ( *y2 * scale * PRECISION ).round() / PRECISION;
      *x  = ( *x  * scale * PRECISION ).round() / PRECISION;
      *y  = ( *y  * scale * PRECISION ).round() / PRECISION; }
    PathSegment::Quadratic { ref mut x1, ref mut y1, ref mut x, ref mut y, .. } => {
      *x1 = ( *x1 * scale * PRECISION ).round() / PRECISION;
      *y1 = ( *y1 * scale * PRECISION ).round() / PRECISION;
      *x  = ( *x  * scale * PRECISION ).round() / PRECISION;
      *y  = ( *y  * scale * PRECISION ).round() / PRECISION; }
    PathSegment::SmoothQuadratic { ref mut x, ref mut y, .. } => {
      *x  = ( *x  * scale * PRECISION ).round() / PRECISION;
      *y  = ( *y  * scale * PRECISION ).round() / PRECISION; }
    PathSegment::EllipticalArc { ref mut x, ref mut y, .. } => {
      *x  = ( *x  * scale * PRECISION ).round() / PRECISION;
      *y  = ( *y  * scale * PRECISION ).round() / PRECISION; }
    PathSegment::ClosePath { .. } => {} }
    }


//==========================================================================================================
// TEXT WRAPPING
//----------------------------------------------------------------------------------------------------------
// use hyphenation::{Language, Load, Standard};

// //----------------------------------------------------------------------------------------------------------
// impl serde::Serialize for textwrap::core::Word {
//     fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
//     where
//         S: serde::Serializer,
//     {
//         serializer.serialize_i32(*self)
//     }
// }

// //----------------------------------------------------------------------------------------------------------
// struct Slab {
//   pub word:             String,
//   pub width:            u16,
//   pub whitespace:       String,
//   pub penalty:          String, }

  // let text = "textwrap: a small library for wrapping text.";
  // let dictionary  = Standard::from_embedded( Language::EnglishUS ).unwrap();
  // let options     = textwrap::Options::new( width ).splitter( dictionary );
  // return format!( "{}", fill( &text, &options ) );
  // return format!( "{:#?}", textwrap::wrap( &text, &options ) );

//----------------------------------------------------------------------------------------------------------
/// return JSON `list<number>` with one wordcount per line
/// TODO: return list of slab indices
#[wasm_bindgen]
pub fn wrap_text( text: String, width: usize ) -> String {
  let words           = textwrap::core::find_words( &text ).collect::<Vec<_>>();
  // urge( &format!( "^827^ words: {:#?}", words ) );
  let lines           = textwrap::core::wrap_optimal_fit( &words, |_| width );
  let mut r: Vec<u16> = Vec::new();
  for line in lines {
    r.push( line.len() as u16 );
    //   let slab = Slab {
    //     word:             tw_word.word,
    //     width:            tw_word.width,
    //     whitespace:       tw_word.whitespace,
    //     penalty:          tw_word.penalty, };
    //   r.push( slab );
  }
  return json!( r ).to_string();
}

