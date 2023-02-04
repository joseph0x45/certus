mod checker;
mod interpreter;
mod runner;
mod utils;

use reqwest;


fn main() -> () {
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
                                    println!("Create template file {}", &argument);
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
                    println!("{}", command)
                }
            }
        },
        None=>{
            println!("Show certus help message");
        }
    }
}
