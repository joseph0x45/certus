extern crate colored;
mod interpreter;
mod runner;
mod utils;

use std::path::Path;
use tokio;
use utils::write_template;
use interpreter::interpreter;

#[tokio::main]
async fn main() -> () {
    let command = std::env::args().nth(1);
    let argument = std::env::args().nth(2);

    match command {
        Some(command)=>{
            match command.as_str() {
                "help"=>{
                    println!("Read certus docs here https://github.com/TheWisePigeon/certus/tree/v0.1.0#readme")
                },
                "generate"=>{
                    match argument {
                        Some(argument)=>{
                            match argument.as_str() {
                                _=>{
                                    if !argument.ends_with(".certus"){
                                        println!("Template file name must have .certus extension");
                                        return
                                    }
                                    let template_path = std::path::Path::new(&argument);
                                    if template_path.exists(){
                                        println!("A file with the same name already exists");
                                        return
                                    }
                                    let mut template_file = std::fs::File::create(template_path).expect("Something went wrong 🥲");
                                    write_template(&mut template_file);
                                    println!("Template file generated");
                                }
                            }
                        },
                        None=>{
                            // let template_path = std::path::Path::new("template.certus");
                            println!("You must provide a name. Read the docs here https://github.com/TheWisePigeon/certus/tree/v0.1.0#readme")
                        }
                    }
                },
                _=>{
                    //User wants to run a file or a directory
                    if Path::new(&command).is_dir(){
                        println!("certus doesn't support running tests in a directory yet. Please pass a certus file instead");
                        return
                    }
                    if !command.ends_with(".certus") {
                        println!("This is not a certus file");
                        return
                    }
                    interpreter(&command).await;
                }
            }
        },
        None=>{
            println!("Learn how to use certus here https://github.com/TheWisePigeon/certus/tree/v0.1.0#readme");
        }
    }
}
