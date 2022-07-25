use std::{
    io::ErrorKind,
    env, 
    fs,
    process::Command, 
    collections::HashSet, 
};
use colored::Colorize;

fn install(flags: HashSet<char>, args: Vec<String>){
    let homeDir = dirs::home_dir()
        .unwrap()
        .into_os_string()
        .into_string()
        .unwrap() + "/.config/alps/";

    fs::create_dir(homeDir.clone()).expect("Failed to create directory!");

    if flags.contains(&'h'){
        println!("usage: {{-I}} [options] [package(s)]");
        println!("options:");
        println!("-f\tinstall file to profile");
        println!("-p\tinstall package to profile")
    } 
    else if flags.contains(&'g'){
        for arg in args{
            match fs::create_dir(homeDir.clone() + &arg){
                Ok(()) => println!("[+] Added group ({}) to config...", arg),
                Err(error) =>{
                    if error.kind() == ErrorKind::AlreadyExists{
                        println!("[!] Group ({}) already exists!", arg);
                    }
                }
            }
        }
    }
    else if flags.contains(&'p'){ 

    }
        
}

fn remove(flags: HashSet<char>, args: Vec<String>){

}

fn sync(flags: HashSet<char>, args: Vec<String>){
    /*let mut packages = String::new();

    for package in args{
        let handle = Command::new("pacman")
            .args(["-Q"])
            .arg(&package)
            .output()
            .expect("Failed to run command.");

        if !handle.status.success(){
            packages += &(package + " ");
        }
        else{
            println!("[!] Package ({}) already exists!", package);
        }
        
    }

    if !packages.is_empty(){
        packages.pop();
        let mut handle = Command::new("sudo");
        let mut command = handle.args(["pacman", "-S"]);

        for substr in packages.as_str().split(' '){
            command = command.arg(&substr);
        }

        command.spawn().expect("Failed to run command");
    }*/
}

fn main(){
    let p_args: Vec<_> = env::args().collect();

    if p_args.len() > 1{
        let mut args: Vec<String> = Vec::new();
        let mut flags = HashSet::new();
        let mut mode = None; 

        for arg in &p_args[1..]{
            if arg.as_bytes()[0] as char == '-'{
                for flag in arg.chars().skip(1){
                    match flag{
                        'I' | 'R' | 'S' =>{
                            if mode == None{
                                mode = Some(flag); 
                            }
                            else{
                                println!(
                                    "{} only one operation may be used at a time",
                                    format!("error:").red()
                                );
                                std::process::exit(1);
                            }
                        }
                        'h' | 'f' | 'p' | 'c' | 'g' => {
                            flags.insert(flag);
                        }
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

        match mode.unwrap(){
            'I' => install(flags, args), 
            'R' => remove(flags, args),
            'S' => sync(flags, args),
            _ => (),
        }
    }
}
