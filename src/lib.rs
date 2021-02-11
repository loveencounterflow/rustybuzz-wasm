
extern crate serde_json;
extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;
// mod cfg;
// mod shape;

#[macro_use]
extern crate serde_derive;

#[wasm_bindgen]
extern {
    pub fn info(s: &str);
    pub fn alert(s: &str);
    pub fn help(s: &str);
    pub fn urge(s: &str);
    // pub message: &str;
}

#[derive(Serialize, Deserialize)]
// acc to https://doc.rust-lang.org/std/fmt/trait.Debug.html
#[derive(Debug)]
pub struct Cfg {
    pub help: bool,
    // pub text: Option<String>,
    pub text: String,
}

impl Cfg {
    pub fn to_string( self: &Self  ) -> String {
        return format!("{:#?}", self );
    }
    pub fn show_details(self: &Self) {
        println!("help: {}", self.help);
        println!("text: {:?}", self.text);
        // println!("The employee name is {}", self.emp_name);
        }
}




#[wasm_bindgen]
pub fn greet( user_cfg: &JsValue ) {
    // // alert( &format!("{}", alert ) );
    // alert( &format!("Hello, {}!", name ) );
    let cfg = Cfg {
        help: true,
        text: String::from( "some text" ),
    };
    let elements: Cfg = user_cfg.into_serde().unwrap();

    // cfg.show_details();
    info( &format!( "^34553^ help: {:#?}", cfg ) );
    help( &format!( "^4575^ {:#?}", &user_cfg ) );
    help( &format!( "^4575^ {:#?}", elements.text ) );
    urge( &format!( "^4575^ {:#?}", elements ) );
    // help( &format!( "^4575^ {:#?}", &user_cfg ) );
    // help( &cfg.to_string() );
    // info( &format!( "^34553^ help: {:?}", cfg.help ));
    // info( &format!( "^34553^ text: {:?}", cfg.text ));
    // // info( &format!( "^34553^ help: {}", cfg ));

}

// #[wasm_bindgen]
// pub fn main( text: str ) -> std::string::String {
//   // shape::shape_text();
//   return format!( "Hello, {}!", text );
// }