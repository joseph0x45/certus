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
            println!("{}", response.status())
        },
        Err(err)=>{
            println!("Request failed {}", err)
        }
    }
    
    
}