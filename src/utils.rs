use std::{fs::File, io::Write, str::FromStr};
use reqwest::{Method, header::HeaderMap};
use serde_json::Value;

pub fn write_template( template_file: &mut File ){
    let template: &str = 
r#"# This is a basic certus test
# The lines preceeded by a '#' are ignored by the interpreter
GET https://jsonplaceholder.typicode.com/todos/1
[HEADERS]
# Here goes your request headers
{

}
[BODY]
{

}
[EXPECT]
# This is where you specify the status code you expect
HTTP 200
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