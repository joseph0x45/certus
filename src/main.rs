extern crate colored;
mod interpreter;
mod runner;
mod utils;

use std::path::Path;
use tokio;
use utils::write_template;
use interpreter::interpreter;
use colored::*;

#[tokio::main]
async fn main() -> () {
    let command = std::env::args().nth(1);
    let argument = std::env::args().nth(2);

    match command {
        Some(command)=>{
            match command.as_str() {
                "help"=>{
                    println!("Show certus help message")
                },
                "generate"=>{
                    println!("Generate certus template file");
                    match argument {
                        Some(argument)=>{
                            match argument.as_str() {
                                _=>{
                                    let template_path = std::path::Path::new(&argument);
                                    if template_path.exists(){
                                        println!("A file with the same name already exists");
                                        return
                                    }
                                    let mut template_file = std::fs::File::create(template_path).expect("Something went wrong 🥲");
                                    write_template(&mut template_file);
                                }
                            }
                        },
                        None=>{
                            // let template_path = std::path::Path::new("template.certus");
                            println!("You must provide a name. Run `certus help` to see certus documentation")
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
            println!("Show certus help message");
        }
    }
}
