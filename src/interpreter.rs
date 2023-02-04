
pub fn interpreter( certus_file: &str ){
    let mut certus_file_content = "";
    let file_reader = std::fs::read_to_string(certus_file);
    match file_reader {
        Ok(content)=>{
            certus_file_content = &content;
            let raw_lines: Vec<&str> = certus_file_content.split("\n").collect();
            println!("{:?}", raw_lines);
        },
        Err(_)=>{
            println!("Something went wrong 🥲\nAre you sure this file exists?");
            return
        }
    }
}