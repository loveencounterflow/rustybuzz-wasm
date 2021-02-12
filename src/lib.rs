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
use std::path::PathBuf;

//==========================================================================================================
// CONFIGURATION
//----------------------------------------------------------------------------------------------------------
#[derive(Serialize, Deserialize)]
// acc to https://doc.rust-lang.org/std/fmt/trait.Debug.html
#[derive(Debug)]
pub struct CfgOpt {
    pub text:             Option<String>,
    // pub font_path:        Option<PathBuf>,
    pub font_bytes_hex:   Option<String>,
    pub font_ptem:        Option<f32>, }

#[derive(Debug)]
pub struct Cfg {
    pub text:             String,
    pub cluster_level:    rustybuzz::BufferClusterLevel,
    pub direction:        rustybuzz::Direction,
    pub face_index:       u32,
    // pub font_path:        PathBuf,
    pub font_bytes:       Vec<u8>,
    pub font_ptem:        Option<f32>,
    pub free:             Vec<String>,
    pub ned:              bool,
    pub no_advances:      bool,
    pub no_clusters:      bool,
    pub no_glyph_names:   bool,
    pub no_positions:     bool,
    pub show_extents:     bool,
    pub show_flags:       bool,
    pub utf8_clusters:    bool,
    pub text_file:        Option<PathBuf>,
    pub unicodes:         Option<String>,
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
  //........................................................................................................
  let font_bytes = match cfg_opt.font_bytes_hex {
    None => vec![],
    Some( x ) => match hex::decode( x ) {
      Ok( v ) => v,
      Err( error ) => {
        alert( &format!( "^89573485^ error decoding hexadecimal: {}", error ) );
        std::process::exit( 1 ); }, }, };
  //........................................................................................................
  let cfg = Cfg {
    text:           match cfg_opt.text { None => String::from( "some text" ), Some( x ) => x, },
    font_bytes:     font_bytes,
    // font_path:      match cfg_opt.font_path { None => PathBuf::from( "somepath" ), Some( x ) => x, },
    // font_path:      cfg_opt.font_path,
    font_ptem:      cfg_opt.font_ptem,
    //......................................................................................................
    language:       rustybuzz::Language::from_str("English").unwrap(),
    //......................................................................................................
    // script:         Some( rustybuzz::Script::new() ),
    script:         None,
    features:       vec![],
    variations:     vec![],
    unicodes:       Some( String::new() ),
    text_file:      Some( PathBuf::new() ),
    show_extents:   false,
    show_flags:     false,
    utf8_clusters:  false,
    no_advances:    false,
    no_clusters:    false,
    no_glyph_names: false,
    no_positions:   false,
    ned:            false, // No Extra Data; Do not output clusters or advances
    free:           vec![],
    direction:      rustybuzz::Direction::LeftToRight,
    cluster_level:  rustybuzz::BufferClusterLevel::MonotoneGraphemes,
    // cluster_level: rustybuzz::BufferClusterLevel::MonotoneCharacters,
    // cluster_level: rustybuzz::BufferClusterLevel::Characters,
    face_index:   0,
  };
  //........................................................................................................
  let mut face = rustybuzz::Face::from_slice(&cfg.font_bytes, cfg.face_index).unwrap();
  face.set_points_per_em( cfg.font_ptem );
  if !cfg.variations.is_empty() { face.set_variations( &cfg.variations ); }
  let mut buffer = rustybuzz::UnicodeBuffer::new();
  buffer.push_str( &cfg.text );
  buffer.set_direction( cfg.direction );
  //........................................................................................................
  buffer.set_language(cfg.language);
  if let Some(script) = cfg.script { buffer.set_script(script); }
  buffer.set_cluster_level(cfg.cluster_level);
  if !cfg.utf8_clusters { buffer.reset_clusters(); }
  //........................................................................................................
  let glyph_buffer = rustybuzz::shape( &face, &cfg.features, buffer );
  //........................................................................................................
  let mut format_flags = rustybuzz::SerializeFlags::default();
  if cfg.no_glyph_names         { format_flags |= rustybuzz::SerializeFlags::NO_GLYPH_NAMES; }
  if cfg.no_clusters || cfg.ned { format_flags |= rustybuzz::SerializeFlags::NO_CLUSTERS;    }
  if cfg.no_positions           { format_flags |= rustybuzz::SerializeFlags::NO_POSITIONS;   }
  if cfg.no_advances || cfg.ned { format_flags |= rustybuzz::SerializeFlags::NO_ADVANCES;    }
  if cfg.show_extents           { format_flags |= rustybuzz::SerializeFlags::GLYPH_EXTENTS;  }
  if cfg.show_flags             { format_flags |= rustybuzz::SerializeFlags::GLYPH_FLAGS;    }
  urge( &format!( "^33321^ {}", glyfs_as_short( &glyph_buffer, &face, format_flags ) ) );
  // let r = glyph_buffer.serialize( &face,  format_flags );
  return glyfs_as_json( &glyph_buffer, &face, format_flags ); }


//==========================================================================================================
//
//----------------------------------------------------------------------------------------------------------
pub fn glyfs_as_json(
  glyph_buffer: &rustybuzz::GlyphBuffer,
  face: &rustybuzz::Face,
  flags: rustybuzz::SerializeFlags ) -> String {
  _glyfs_as_json( &glyph_buffer, face, flags ).unwrap_or_default() }

//----------------------------------------------------------------------------------------------------------
fn _glyfs_as_json(
  glyph_buffer: &rustybuzz::GlyphBuffer,
  face: &rustybuzz::Face,
  flags: rustybuzz::SerializeFlags) -> Result<String, std::fmt::Error> {
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
pub fn glyfs_as_short(
  glyph_buffer: &rustybuzz::GlyphBuffer,
  face: &rustybuzz::Face,
  flags: rustybuzz::SerializeFlags ) -> String {
  _glyfs_as_short( &glyph_buffer, face, flags ).unwrap_or_default() }

//----------------------------------------------------------------------------------------------------------
fn _glyfs_as_short(
  glyph_buffer: &rustybuzz::GlyphBuffer,
  face: &rustybuzz::Face,
  flags: rustybuzz::SerializeFlags) -> Result<String, std::fmt::Error> {
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

