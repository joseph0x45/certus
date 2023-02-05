use std::str::FromStr;

use crate::utils::CertusTest;
use serde_json::{Value, Error};
use reqwest::Method;
use crate::runner::run;

pub async fn interpreter( certus_file: &str ){
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
            let test_method = Method::from_str(verb_and_endpoint[0]);
            match test_method {
                Ok(method)=>{
                    certus_test.method = method;
                },
                Err(_)=>{
                    println!("Invalid HTTP method {}", verb_and_endpoint[0]);
                    return
                }
            }
            certus_test.endpoint = verb_and_endpoint[1].to_string();
            //Retrieve headers
            let mut headers = "".to_string();
            let mut read_headers = false;
            for line in content.lines(){
                if line.trim()=="[BODY]" {
                    break;
                }
                if line.trim()=="[HEADERS]"{
                    read_headers = true;
                    continue;
                }
                if (line.trim()!="[HEADERS]") & (read_headers==false) {
                    continue;
                }
                if line.trim().starts_with("#") || line.trim()==""{
                    continue;
                }
                headers.push_str(line);
                headers.push_str("\n");
            }
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
                    certus_test.body = json_body;
                },
                Err(_)=>{
                    println!("Body serialization failed. Make sure you wrote a valid JSON");
                    return
                }
            }
            run(certus_test).await;
        },
        Err(_)=>{
            println!("Something went wrong 🥲\nAre you sure this file exists?");
            return
        }
    }
}