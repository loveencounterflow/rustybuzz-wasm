use wasm_bindgen::prelude::*;
mod cfg;
// mod shape;


#[wasm_bindgen]
extern {
    pub fn info(s: &str);
    pub fn alert(s: &str);
    pub fn help(s: &str);
    pub fn urge(s: &str);
    // pub message: &str;
}

#[wasm_bindgen]
pub fn greet( name: &str ) {
    // alert( &format!("{}", alert ) );
    alert( &format!("Hello, {}!", name ) );
    let cfg = cfg::Cfg {
        help: true,
        text: String::from( "some text" ),
    };
    cfg.show_details();
    info( &format!( "^34553^ help: {:#?}", cfg ) );
    help( &cfg.to_string() );
    info( &format!( "^34553^ help: {:?}", cfg.help ));
    info( &format!( "^34553^ text: {:?}", cfg.text ));
    // info( &format!( "^34553^ help: {}", cfg ));

}

// #[wasm_bindgen]
// pub fn main( text: str ) -> std::string::String {
//   // shape::shape_text();
//   return format!( "Hello, {}!", text );
// }