use std::env;
use colored::Colorize;

fn main(){
    let p_args: Vec<_> = env::args().collect();

    if p_args.len() > 1{
        let mut args: Vec<String> = Vec::new();
        let mut flags: Vec<char> = Vec::new();
        let mut mode = None; 

        for arg in &p_args[1..]{
            if arg.as_bytes()[0] as char == '-'{
                for flag in &arg.as_bytes()[1..]{
                    match *flag as char{
                        'I' | 'R' | 'S' =>{
                            if mode == None{
                                mode = Some(*flag as char); 
                            }
                            else{
                                println!(
                                    "{} only one operation may be used at a time",
                                    format!("error:").red()
                                );
                                std::process::exit(1);
                            }
                        }
                        'h' | 'f' | 'p' | 'c' => flags.push(*flag as char),
                        op =>{ 
                            println!("{} invalid option -- '{}'",
                                format!("error:").red(),
                                op    
                            );
                            std::process::exit(1);
                        }
                    }
                }
                continue;
            }
            args.push(arg.clone());
        }
    }
}
