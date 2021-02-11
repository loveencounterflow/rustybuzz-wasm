
extern crate serde_json;
extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;
// mod cfg;
// mod shape;

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
// use std::str::FromStr;

//==========================================================================================================
// CONFIGURATION
//----------------------------------------------------------------------------------------------------------
#[derive(Serialize, Deserialize)]
// acc to https://doc.rust-lang.org/std/fmt/trait.Debug.html
#[derive(Debug)]
pub struct CfgOpt {
    pub text:             Option<String>,
    pub font_file:        Option<PathBuf>,
    pub font_ptem:        Option<f32>,
}

// #[derive(Serialize, Deserialize)]
#[derive(Debug)]
pub struct Cfg {
    pub text:             String,
    pub cluster_level:    rustybuzz::BufferClusterLevel,
    pub direction:        rustybuzz::Direction,
    pub face_index:       u32,
    pub font_file:        Option<PathBuf>,
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
    // pub features:         Vec<rustybuzz::Feature>,
    // pub language:         rustybuzz::Language,
    // pub script:           rustybuzz::Script,
    //......................................................................................................
    // not implemented:
    // pub help:             bool,
    // pub version:          bool,
}


//==========================================================================================================
//
//----------------------------------------------------------------------------------------------------------
// fn parse_features(s: &str) -> Result<Vec<rustybuzz::Feature>, String> {
//     let mut features = Vec::new();
//     for f in s.split(',') {
//         features.push(rustybuzz::Feature::from_str(&f)?);
//     }
//     Ok(features)
// }

// fn system_language() -> rustybuzz::Language {
//     unsafe {
//         libc::setlocale(libc::LC_ALL, b"\0" as *const _ as *const i8);
//         let s = libc::setlocale(libc::LC_CTYPE, std::ptr::null());
//         let s = std::ffi::CStr::from_ptr(s);
//         let s = s.to_str().expect("locale must be ASCII");
//         rustybuzz::Language::from_str(s).unwrap()
//     }
// }

// //----------------------------------------------------------------------------------------------------------
// fn parse_variations(s: &str) -> Result<Vec<rustybuzz::Variation>, String> {
//     let mut variations = Vec::new();
//     for v in s.split(',') {
//         variations.push(rustybuzz::Variation::from_str(&v)?);
//     }
//     Ok(variations)
// }


//==========================================================================================================
//
//----------------------------------------------------------------------------------------------------------
#[wasm_bindgen]
pub fn greet( user_cfg: &JsValue ) {
  // help( &format!( "^4575^ {:#?}", &user_cfg ) );
  //........................................................................................................
  let cfg_opt: CfgOpt = user_cfg.into_serde().unwrap();
  let cfg = Cfg {
    text:           match cfg_opt.text      { None => String::from( "some text" ), Some( x ) => x, },
    font_file:      cfg_opt.font_file,
    // font_file:      match cfg_opt.font_file { None => Some( PathBuf::from( "/tmp/foo.ttf" )), Some( x ) => Some( x ), },
    font_ptem:      cfg_opt.font_ptem,
    // font_ptem:      match cfg_opt.font_ptem { None => 42.0, Some( x ) => x, },
    //......................................................................................................
    // features:       parse_features( "liga" )?.unwrap_or_default(),
    // language:       rustybuzz::Language::from( "English" ),
    //......................................................................................................
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
  urge( &format!( "^4575^ {:#?}", cfg ) );
}

// #[wasm_bindgen]
// pub fn main( text: str ) -> std::string::String {
//   // shape::shape_text();
//   return format!( "Hello, {}!", text );
// }



