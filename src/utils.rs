use std::{fs::File, io::Write, str::FromStr};
use reqwest::{Method, header::HeaderMap};
use serde_json::Value;

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
    pub method: Method,
    pub endpoint: String,
    pub headers: HeaderMap,
    pub body: Value,
    pub expected_status: u16,
    pub expected_headers: HeaderMap,
    pub expected_body: Value,

}

impl Default for CertusTest {
    fn default() -> Self {
        CertusTest { 
            method: Method::from_str("GET").unwrap(), 
            endpoint: "".to_string(), 
            headers: HeaderMap::new(), 
            body: Value::default(), 
            expected_status:200,
            expected_headers: HeaderMap::new(),
            expected_body: Value::default() 
        }
    }
}