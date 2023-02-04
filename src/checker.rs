
pub fn valid_verb_and_endpoint( verb: &str, endpoint: &str ) -> bool {
    match verb {
        "get" | "GET" | "post" | "POST" | "put" | "PUT" | "patch" | "PATCH" | "delete" | "DELETE" | "options" | "OPTIONS"=>{
            
        },
        _=>{
            println!("Unrecognized HTTP verb {}", verb);
            return false
        }
    }
    if !(endpoint.starts_with("http://") || endpoint.starts_with("https://")){
        println!("Invalid url {}", endpoint);
        return false
    }
    true
}
