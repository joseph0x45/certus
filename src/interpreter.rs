use crate::utils::CertusTest;
use crate::checker::{valid_verb_and_endpoint};
use serde_json::{Value, Error};

pub fn interpreter( certus_file: &str ){
    let file_reader = std::fs::read_to_string(certus_file);
    let mut certus_test = CertusTest::default();
    match file_reader {
        Ok(content)=>{
            let directives: Vec<&str> = content.split("\n").collect();
            let cleaned_directives: Vec<_> = directives
            .iter()
            .filter(|directive|{
                !directive.starts_with("#") & ( directive!=&&"")
            }).collect::<Vec<_>>();
            //Interpret first line to extract verb an dendpoint
            let mut first_line = cleaned_directives[0].to_string();
            first_line = first_line.trim().to_string();
            let verb_and_endpoint: Vec<&str> = first_line.split_whitespace().collect();

            if verb_and_endpoint.len()!=2{
                println!("Error while reading verb and endpoint. Please refeer to the documentation to see how to write certus files");
                return
            }
            if !valid_verb_and_endpoint(verb_and_endpoint[0], verb_and_endpoint[1]){
                return
            }
            certus_test.verb = verb_and_endpoint[0].to_string();
            certus_test.endpoint = verb_and_endpoint[1].to_string();

            //Retrieve body
            let mut body = "".to_string();
            let mut read_body: bool = false;
            for line in content.lines(){
                if line.trim()=="[EXPECT]"{
                    break;
                }
                if line.trim()=="[BODY]"{
                    read_body = true;
                    continue;
                }
                if (line.trim()!="[BODY]") & (read_body==false) {
                    continue;
                }
                body.push_str(line);
            }
            let value: Result<Value, Error> = serde_json::from_str(body.as_str());
            match value {
                Ok(json_body)=>{
                    println!("{:?}", json_body)
                },
                Err(_)=>{
                    println!("Body serialization failed. Make sure you wrote a valid JSON");
                    return
                }
            }
        },
        Err(_)=>{
            println!("Something went wrong 🥲\nAre you sure this file exists?");
            return
        }
    }
}