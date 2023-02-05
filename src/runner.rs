use crate::utils::CertusTest;
use reqwest::Client;

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
            let response_status = response.status();
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