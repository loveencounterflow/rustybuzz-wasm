
extern crate serde_json;
extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;
// mod cfg;
// mod shape;

// #[wasm_bindgen]
// pub fn init_panic_hook() {
//     console_error_panic_hook::set_once();
// }

// use std::panic;
// #[macro_use]
// extern crate stdweb;

// fn main() {
//     panic::set_hook(Box::new(|p| {
//         let s = p.to_string();
//         js!{ console.error(@{s});}
//     }));
// }

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
    pub font_path:        Option<PathBuf>,
    pub font_ptem:        Option<f32>,
}

// #[derive(Serialize, Deserialize)]
#[derive(Debug)]
pub struct Cfg {
    pub text:             Option<String>,
    pub cluster_level:    rustybuzz::BufferClusterLevel,
    pub direction:        rustybuzz::Direction,
    pub face_index:       u32,
    pub font_path:        PathBuf,
    // pub font_path:        Option<PathBuf>,
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
    pub language:         rustybuzz::Language,
}


//==========================================================================================================
//
//----------------------------------------------------------------------------------------------------------
#[wasm_bindgen]
pub fn shape_text( user_cfg: &JsValue ) {
  // help( &format!( "^4575^ {:#?}", &user_cfg ) );
  //........................................................................................................
  let cfg_opt: CfgOpt = user_cfg.into_serde().unwrap();
  let cfg = Cfg {
    text:           cfg_opt.text,
    font_path:      match cfg_opt.font_path { None => PathBuf::from( "somepath" ), Some( x ) => x, },
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
  urge( &format!( "^4575^ {:#?}", cfg ) );

  // let mut font_set_as_free_arg = false;
  // let font_path = if let Some(path) = cfg.font_path {
  //   path.clone()
  // } else if !cfg.free.is_empty() {
  //   font_set_as_free_arg = true;
  //   PathBuf::from(&cfg.free[0])
  // } else {
  //   eprintln!("Error: font is not set.");
  //   std::process::exit(1);
  // };

  alert( "^3334-1^" );
  urge( &format!( "^43447^ {:#?}", cfg.font_path )  );
  urge( &format!( "^43447^ {:#?}", cfg.font_path.exists() )  );
  // urge( &format!( "^43447^ {:#?}", font_path )  );
  // urge( &format!( "^43447^ {:#?}", font_path.exists() )  );
  return;
  // if !font_path.exists() {
  //   alert( &format!( "Error: '{}' does not exist.", font_path.display() ) );
  //   std::process::exit(1);
  // }


  /*
  alert( "^3334-2^" );

  let font_data = std::fs::read(font_path).unwrap();
  alert( "^3334-3^" );
  let mut face = rustybuzz::Face::from_slice(&font_data, cfg.face_index).unwrap();
  alert( "^3334-4^" );

  alert( "^3334-5^" );
  face.set_points_per_em(cfg.font_ptem);

  if !cfg.variations.is_empty() {
    face.set_variations(&cfg.variations);
  }

  let text = if let Some(path) = cfg.text_file {
    std::fs::read_to_string(path).unwrap()
  } else if cfg.free.len() == 2 && font_set_as_free_arg {
    cfg.free[1].clone()
  } else if cfg.free.len() == 1 && !font_set_as_free_arg {
    cfg.free[0].clone()
  } else if let Some(ref text) = cfg.unicodes {
    text.clone()
  } else if let Some(ref text) = cfg.text {
    text.clone()
  } else {
    eprintln!("Error: text is not set.");
    std::process::exit(1);
  };

  let mut buffer = rustybuzz::UnicodeBuffer::new();
  buffer.push_str(&text);

    buffer.set_direction(cfg.direction);
  // if let Some(d) = cfg.direction {
    // buffer.set_direction(d);
  // }

  alert( "^3334-6^" );
  buffer.set_language(cfg.language);

  if let Some(script) = cfg.script {
    buffer.set_script(script);
  }

  buffer.set_cluster_level(cfg.cluster_level);

  if !cfg.utf8_clusters {
    buffer.reset_clusters();
  }

  let glyph_buffer = rustybuzz::shape(&face, &cfg.features, buffer);

  let mut format_flags = rustybuzz::SerializeFlags::default();
  if cfg.no_glyph_names {
    format_flags |= rustybuzz::SerializeFlags::NO_GLYPH_NAMES;
  }

  alert( "^3334-7^" );
  if cfg.no_clusters || cfg.ned {
    format_flags |= rustybuzz::SerializeFlags::NO_CLUSTERS;
  }

  if cfg.no_positions {
    format_flags |= rustybuzz::SerializeFlags::NO_POSITIONS;
  }

  if cfg.no_advances || cfg.ned {
    format_flags |= rustybuzz::SerializeFlags::NO_ADVANCES;
  }

  if cfg.show_extents {
    format_flags |= rustybuzz::SerializeFlags::GLYPH_EXTENTS;
  }

  if cfg.show_flags {
    format_flags |= rustybuzz::SerializeFlags::GLYPH_FLAGS;
  }
  alert( "^3334-8^" );
  // info( &format!( "{}", glyph_buffer.serialize(&face,  format_flags ) ) );
*/
}

// //==========================================================================================================
// //
// //----------------------------------------------------------------------------------------------------------
// fn parse_unicodes(s: &str) -> Result<String, String> {
//   use std::convert::TryFrom;

//   let mut text = String::new();
//   for u in s.split(',') {
//     let u = u32::from_str_radix(&u[2..], 16)
//       .map_err(|_| format!("'{}' is not a valid codepoint", u))?;

//     let c = char::try_from(u).map_err(|_| format!("{} is not a valid codepoint", u))?;

//     text.push(c);
//   }

//   Ok(text)
// }

// fn parse_features(s: &str) -> Result<Vec<rustybuzz::Feature>, String> {
//   let mut features = Vec::new();
//   for f in s.split(',') {
//     features.push(rustybuzz::Feature::from_str(&f)?);
//   }

//   Ok(features)
// }

// fn parse_variations(s: &str) -> Result<Vec<rustybuzz::Variation>, String> {
//   let mut variations = Vec::new();
//   for v in s.split(',') {
//     variations.push(rustybuzz::Variation::from_str(&v)?);
//   }

//   Ok(variations)
// }

// fn parse_cluster(s: &str) -> Result<rustybuzz::BufferClusterLevel, String> {
//   match s {
//     "0" => Ok(rustybuzz::BufferClusterLevel::MonotoneGraphemes),
//     "1" => Ok(rustybuzz::BufferClusterLevel::MonotoneCharacters),
//     "2" => Ok(rustybuzz::BufferClusterLevel::Characters),
//     _ => Err(format!("invalid cluster level"))
//   }
// }

// fn system_language() -> rustybuzz::Language {
//   unsafe {
//     libc::setlocale(libc::LC_ALL, b"\0" as *const _ as *const i8);
//     let s = libc::setlocale(libc::LC_CTYPE, std::ptr::null());
//     let s = std::ffi::CStr::from_ptr(s);
//     let s = s.to_str().expect("locale must be ASCII");
//     rustybuzz::Language::from_str(s).unwrap()
//   }
// }
