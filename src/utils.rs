use std::{fs::File, io::Write, collections::HashMap};



pub fn write_template( template_file: &mut File ){
    let template: &str = 
r#"# This is a basic certus test
# The lines preceeded by a '#' are ignored by the interpreter
GET https://jsonplaceholder.typicode.com/todos/1
[CFG]
# Here goes all the configuration for your request, such as headers and body
[EXPECT]
# This is where you specify the status code you expect and what certus must do when it gets such codes
HTTP 200 
# You can access the body and header of the response and make assertions on it for further test validation
# HTTP 404
# body.data: "something not found"
"#;
    template_file.write(template.as_bytes()).expect("Something went wrong 🥲");
}

pub struct CertusTest{
    pub verb: String,
    pub endpoint: String,
    pub header: HashMap<String, String>

}

impl Default for CertusTest {
    fn default() -> Self {
        CertusTest { verb: "".to_string(), endpoint: "".to_string(), header: HashMap::new() }
    }
}