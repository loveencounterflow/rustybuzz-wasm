// #![allow(dead_code)]
// #![allow(unused_variables)]

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


//==========================================================================================================
// PERSISTENT STATE
//----------------------------------------------------------------------------------------------------------
// thx to https://stackoverflow.com/a/19608953/256361
static mut COUNT: i32 = 0;

#[wasm_bindgen]
pub fn inc() -> i32 {
unsafe {
  COUNT += 1;
  return COUNT; }; }

#[wasm_bindgen]
pub fn dec() -> i32 {
unsafe {
  COUNT += -1;
  return COUNT; }; }

//----------------------------------------------------------------------------------------------------------
// thx to https://stackoverflow.com/a/19608953/256361
static mut FONT_BYTES: Vec<u8> = vec![];

#[wasm_bindgen]
pub fn set_font_bytes( font_bytes_hex: String ) {
  unsafe { FONT_BYTES = match hex::decode( font_bytes_hex ) {
      Ok( v ) => v,
      Err( error ) => {
        alert( &format!( "^895734^ error decoding hexadecimal: {}", error ) );
        std::process::exit( 1 ); }, }; }; }

#[wasm_bindgen]
pub fn has_font_bytes() -> bool { unsafe { !FONT_BYTES.is_empty() } }


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
  let mut face = unsafe { rustybuzz::Face::from_slice(&FONT_BYTES, cfg.face_index).unwrap() };
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

