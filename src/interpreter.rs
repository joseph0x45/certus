use std::str::FromStr;
use std::collections::HashMap;
use crate::utils::CertusTest;
use serde_json::{Value, Error};
use reqwest::{Method, header::{HeaderValue, HeaderName}};
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
            }
            if headers==""{
                headers.push_str("{}");
            }
            let value: Result<Value, Error> = serde_json::from_str(headers.as_str());
            match value {
                Ok(headers)=>{
                    let mut headers_hashmap: HashMap<String, String> = HashMap::default();
                    if let Value::Object(map) = headers{
                        for(key, val) in map{
                            headers_hashmap.insert(key, val.as_str().unwrap().to_string());
                        }
                    }
                    for (key, val) in headers_hashmap{
                        let header_name = HeaderName::from_str(key.as_str());
                        match header_name {
                            Ok(valid_header_name)=>{
                                let header_value = HeaderValue::from_str(val.as_str());
                                match header_value {
                                    Ok(valid_header_value)=>{
                                        certus_test.headers.insert(valid_header_name, valid_header_value);
                                    },
                                    Err(_)=>{
                                        println!("Invalid header value {}", &val);
                                        return
                                    }
                                }
                            },
                            Err(_)=>{
                                println!("Invalid header name {}", &key);
                                return
                            }
                        }
                    }
                },
                Err(_)=>{
                    println!("Failed to parse headers.");
                    return;
                }
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
            if body==""{
                body.push_str("{}");
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
            // println!("Here");
            //Retrieve expected HTTP status
            let mut read_status = false;
            for line in content.lines(){
                if line.trim()=="[EXPECT]" {
                    read_status = true;
                    continue;
                }
                if line.trim().starts_with("#") {
                    continue;
                }
                if line.trim()==""{
                    continue;
                }
                if line.trim().starts_with("HTTP") & read_status{
                    let splitted : Vec<&str> = line.split(" ").collect();
                    if splitted.len()==2{
                        match u16::from_str(splitted[1])  {
                            Ok(status)=>{
                                certus_test.expected_status = status;
                                break;
                            },
                            Err(_)=>{
                                println!("Invalid HTTP status code {}", splitted[1]);
                                return ;
                            }
                        }
                    }
                    println!("Failed to parse expected HTTP status code");
                    return;
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