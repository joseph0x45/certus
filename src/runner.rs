use crate::utils::CertusTest;
use reqwest::Client;
use serde_json::json;

pub async fn run( test: CertusTest ){
    let reqwest_client = Client::new();
    let method = test.method;
    let certus_request = reqwest_client
        .request(method, test.endpoint)
        .headers(test.headers)
        .body(test.body.to_string());
    
    
    
    
}