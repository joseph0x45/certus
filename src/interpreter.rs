pub fn interpreter( certus_file: &str ){
    let mut certus_file_content = "";
    let file_reader = std::fs::read_to_string(certus_file);
    match file_reader {
        Ok(content)=>{
            println!("{}", content);
            certus_file_content = &content;
        },
        Err(_)=>{
            println!("Something went wrong 🥲\nAre you sure this file exists?");
            return
        }
    }
}