
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

//==========================================================================================================
// CONFIGURATION
//----------------------------------------------------------------------------------------------------------
#[derive(Serialize, Deserialize)]
// acc to https://doc.rust-lang.org/std/fmt/trait.Debug.html
#[derive(Debug)]
pub struct CfgOpt {
    pub help:    Option<bool>,
    pub text:    Option<String>,
    pub perhaps: Option<u32>,
    // pub perhaps: u32,
}

// #[derive(Serialize, Deserialize)]
#[derive(Debug)]
pub struct Cfg {
    pub help: bool,
    pub text: String,
    pub perhaps: u32,
    // pub perhaps: u32,
}


//==========================================================================================================
//
//----------------------------------------------------------------------------------------------------------
#[wasm_bindgen]
pub fn greet( user_cfg: &JsValue ) {
  // help( &format!( "^4575^ {:#?}", &user_cfg ) );
  //......................................................................................................
  let cfg_opt: CfgOpt = user_cfg.into_serde().unwrap();
  let cfg = Cfg {
    help:    match cfg_opt.help    { None => false,                       Some( x ) => x, },
    text:    match cfg_opt.text    { None => String::from( "some text" ), Some( x ) => x, },
    perhaps: match cfg_opt.perhaps { None => 42,                          Some( x ) => x, }, };
  urge( &format!( "^4575^ {:#?}", cfg ) );
}

// #[wasm_bindgen]
// pub fn main( text: str ) -> std::string::String {
//   // shape::shape_text();
//   return format!( "Hello, {}!", text );
// }



