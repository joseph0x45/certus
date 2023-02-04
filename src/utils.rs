use std::{fs::File, io::Write};



pub fn write_template( template_file: &mut File ){
    let template: &str = 
r#"# This is a basic certus test
# The lines preceeded by a '#' are ignored by the interpreter
GET https://jsonplaceholder.typicode.com/todos/1
[CFG]
# Here goes all the configuration for your request, such as headers and body
[EXPECT]
HTTP 200 
"#;
    template_file.write(template.as_bytes()).expect("Something went wrong 🥲");
}