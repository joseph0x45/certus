use crate::utils::CertusTest;
use reqwest::Client;
use colored::*;

pub async fn run( test: CertusTest ){
    let reqwest_client = Client::new();
    let method = test.method;
    let certus_request = reqwest_client
        .request(method, test.endpoint)
        .headers(test.headers)
        .body(test.body.to_string());
    let response = certus_request.send().await;
    match response {
        Ok(response)=>{
            let response_status = response.status().as_u16();
            if response_status!=test.expected_status {
                let error_message = std::format!("Test failed: Got HTTP {} but expected HTTP {}", response_status, test.expected_status).color("red");                
                println!("{}", error_message);
                return
            }
            
            let response_headers = response.headers();
            let response_text = &response.text().await;
            match response_text {
                Ok(response)=>{
                    
                },
                Err(err)=>{
                    println!("Failed to parse response text {}", err);
                }
            }
        },
        Err(err)=>{
            println!("Request failed {}", err)
        }
    }
    
    
}