

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

