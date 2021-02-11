
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
pub struct CfgOpt {
    pub help: Option<bool>,
    pub text: Option<String>,
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

// impl CfgOpt {
//     pub fn to_string( self: &Self  ) -> String {
//         return format!("{:#?}", self );
//     }
//     pub fn show_details(self: &Self) {
//         println!("help: {}", self.help);
//         println!("text: {:?}", self.text);
//         // println!("The employee name is {}", self.emp_name);
//         }
// }


//----------------------------------------------------------------------------------------------------------
#[wasm_bindgen]
pub fn greet( user_cfg: &JsValue ) {
    // // alert( &format!("{}", alert ) );
    // alert( &format!("Hello, {}!", name ) );
    //......................................................................................................
    // let cfg_1 = CfgOpt {
    //     help: true,
    //     text: String::from( "some text" ),
    //     // extra: 108,
    //     perhaps: Some( 111 ),
    // };
    // info( &format!( "^34553^ help: {:#?}", cfg_1 ) );
    //......................................................................................................
    help( &format!( "^4575^ {:#?}", &user_cfg ) );
    demo_options();
    //......................................................................................................
    let cfg_opt: CfgOpt = user_cfg.into_serde().unwrap();
    let cfg = Cfg {
        help: match cfg_opt.help { None => false, Some( x ) => x, },
        text: match cfg_opt.text { None => String::from( "some text" ), Some( x ) => x, },
        perhaps: match cfg_opt.perhaps { None => 42, Some( x ) => x, },

    };
    // cfg_opt.text = match cfg_opt.text {
    //     None            => String::from( "default text" ),
    //     Some( ref x )   => x.to_string(),
    // };
    // let mut cfg: Cfg;
    // // cfg.perhaps = cfg.perhaps.unwrap();
    // help( &format!( "^4575^ {:#?}", cfg.text ) );
    urge( &format!( "^4575^ {:#?}", cfg ) );
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


//##########################################################################################################
struct FullNameOption {
    first: String,
    middle: Option<String>,
    last: String,
}

struct FullName {
    first: String,
    middle: String,
    last: String,
}


fn demo_options() {
    let alice = FullNameOption {
        first: String::from("Alice"),
        middle: Some(String::from("Bob")), // Alice has a middle name
        last: String::from("Smith")
    };

    let jon = FullNameOption {
        first: String::from("Jon"),
        middle: None, // Jon has no middle name
        last: String::from("Snow")
    };
    // let alice_middle    = alice.middle.unwrap();
    // let jon_middle      = match jon.middle {
    //     None => "No middle name!",
    //     Some( ref x ) => x,
    //     };
    // help( &format!( "^129^ {} - {} - {}", alice.first, alice_middle, alice.last ) );
    // help( &format!( "^129^ {} - {} - {}", alice.first, alice_middle, alice.last ) );
    // help( &format!( "^129^ {} - {} - {}", jon.first,   jon_middle, jon.last ) );
    // // help( format!( "Alice's middle name is {}", alice.middle.unwrap() ).as_ref() );
    // help( &format!( "Jon's middle name is {}", jon_middle ) );
    let alice_resolved = FullName {
        first: alice.first,
        middle: match alice.middle {
            None => String::from( "No middle name!" ),
            Some( x ) => x,
            },
        last: alice.last,

    };
    help( &format!( "^129^ {} - {} - {}", alice_resolved.first, alice_resolved.middle, alice_resolved.last ) );
}

