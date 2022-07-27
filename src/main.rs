use std::{
    io::{self, prelude::*, BufReader, ErrorKind},
    env, 
    fs::{self, File},
    process::Command, 
    collections::HashSet, 
    path::Path, ops::RangeBounds,
};
use colored::Colorize;

fn install(flags: HashSet<char>, args: Vec<String>){
    let homeDir = dirs::home_dir()
        .unwrap()
        .into_os_string()
        .into_string()
        .unwrap() + "/.config/alps/";

    fs::create_dir(homeDir.clone());

    if flags.contains(&'h'){
        println!("usage: {{-I}} [options] [package(s)]");
        println!("options:");
        println!("-f\tinstall file to profile");
        println!("-p\tinstall package to profile")
    } 
    else if flags.contains(&'g'){
        for arg in args{
            match fs::create_dir(homeDir.clone() + &arg){
                Ok(()) => println!("[+] Created group ({})...", arg),
                Err(error) =>{
                    if error.kind() == ErrorKind::AlreadyExists{
                        println!("[!] Group ({}) already exists!", arg);
                    }
                }
            }
        }
    }
    else if flags.contains(&'p') && !args.is_empty(){ 
        if Path::new(&(homeDir.clone() + &args[0])).is_dir(){
            let mut handle = fs::OpenOptions::new()
                .write(true)
                .read(true)
                .create(true)
                .open(homeDir.clone() + &args[0] + "/" + &args[0] + ".conf")
                .unwrap();

            for package in &args[1..]{
                handle.rewind().unwrap();
                let mut text = String::new();
                let mut uText = String::new();
                handle.read_to_string(&mut text).unwrap();

                let mut segments: Vec<&str> = text.split("[PACKAGES]").collect();
                if segments.len() < 2{
                    handle.seek(std::io::SeekFrom::End(0));
                    handle.write_all(b"[PACKAGES]").unwrap();
                }
                handle.rewind().unwrap();
                handle.read_to_string(&mut uText).unwrap();
                segments = uText.split("[PACKAGES]").collect();

                match segments.len(){
                    3.. => panic!("Redefinition of label [PACKAGES]"),
                    2 =>{
                        let packages = segments[1].split("[PATHS]").next().unwrap();

                        if packages.contains(package.as_str()){
                            println!("[.] Package ({}) already added to group ({})!", package, args[0]);
                        }
                        else{
                            handle.rewind().unwrap();
                            handle.write_all(
                                (segments[0].to_owned()
                                + "[PACKAGES]\n" 
                                + &package
                                + " "
                                + segments[1]
                            ).as_bytes()).unwrap();
                            println!("[+] Added package ({}) to group ({})...", package, args[0]);
                        }
                    }
                    _ =>(),
                }           
            }
        }
        else{
            println!("[!] Group ({}) does not exist! (use -h for help)", args[0]);
        }
    }
    else{
        println!(
            "{} invalid option! (use -h for help)",
            "error:".red()
         );
         std::process::exit(1);
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

fn list(flags: HashSet<char>, args: Vec<String>){
    if flags.contains(&'h'){
        println!("usage: {{-L}} [options]"); 
        println!("options:");
        println!("-f\tinstall file to profile");
        println!("-p\tinstall package to profile")
    }
    else if flags.contains(&'g'){
        println!("g");
    }
    else if flags.contains(&'p'){
        println!("p");
    }
    else{
        println!(
            "{} invalid option! (use -h for help)",
            "error:".red()
         );
         std::process::exit(1);
    }
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
                        'I' | 'R' | 'S' | 'L' =>{
                            if mode == None{
                                mode = Some(flag); 
                            }
                            else{
                                println!(
                                    "{} only one operation may be used at a time",
                                    "error:".red()
                                );
                                std::process::exit(1);
                            }
                        }
                        'h' | 'f' | 'p' | 'c' | 'g' => {
                            flags.insert(flag);
                        }
                        op =>{ 
                            println!("{} invalid option -- '{}'",
                                "error:".red(),
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
            'L' => list(flags, args),
            _ => (),
        }
    }
}
