
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
}

// #[derive(Serialize, Deserialize)]
#[derive(Debug)]
pub struct Cfg {
    pub text:             String,
    pub cluster_level:    rustybuzz::BufferClusterLevel,
    // pub direction:        rustybuzz::Direction,
    pub face_index:       u32,
    // pub features:         Vec<rustybuzz::Feature>,
    // pub font_file:        PathBuf,
    // pub font_ptem:        f32,
    // pub free:             Vec<String>,
    // pub language:         rustybuzz::Language,
    // pub ned:              bool,
    // pub no_advances:      bool,
    // pub no_clusters:      bool,
    // pub no_glyph_names:   bool,
    // pub no_positions:     bool,
    // pub script:           rustybuzz::Script,
    // pub show_extents:     bool,
    // pub show_flags:       bool,
    // pub text:             String,
    // pub text_file:        PathBuf,
    // pub unicodes:         String,
    // pub utf8_clusters:    bool,
    // pub variations:       Vec<rustybuzz::Variation>,
    //......................................................................................................
    // not implemented:
    // pub help:             bool,
    // pub version:          bool,
}


//==========================================================================================================
//
//----------------------------------------------------------------------------------------------------------
#[wasm_bindgen]
pub fn greet( user_cfg: &JsValue ) {
  // help( &format!( "^4575^ {:#?}", &user_cfg ) );
  //........................................................................................................
  let cfg_opt: CfgOpt = user_cfg.into_serde().unwrap();
  let cfg = Cfg {
    text:         match cfg_opt.text    { None => String::from( "some text" ), Some( x ) => x, },
    cluster_level: rustybuzz::BufferClusterLevel::MonotoneGraphemes,
    // cluster_level: rustybuzz::BufferClusterLevel::MonotoneCharacters,
    // cluster_level: rustybuzz::BufferClusterLevel::Characters,
    //......................................................................................................
    face_index:   0,
  };
  urge( &format!( "^4575^ {:#?}", cfg ) );
}

// #[wasm_bindgen]
// pub fn main( text: str ) -> std::string::String {
//   // shape::shape_text();
//   return format!( "Hello, {}!", text );
// }



