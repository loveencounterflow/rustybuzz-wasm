use wasm_bindgen::prelude::*;
// mod shape;

#[wasm_bindgen]
extern {
    pub fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello, {}!", name));
}

// #[wasm_bindgen]
// pub fn main( text: str ) -> std::string::String {
//   // shape::shape_text();
//   return format!( "Hello, {}!", text );
// }