use std::{
    fs,
    env, 
    path::Path, 
    process::Command,
    collections::HashSet, 
    io::{prelude::*, ErrorKind},
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
                Ok(()) => println!("{} Created group ({})...", "[+]".green(), arg.green()),
                Err(error) =>{
                    if error.kind() == ErrorKind::AlreadyExists{
                        println!("{} Group ({}) already exists!", "[!]".yellow(), arg.yellow());
                    }
                }
            }
        }
    }
    else if flags.contains(&'p'){ 
        if !args.is_empty(){
            if Path::new(&(homeDir.clone() + &args[0])).is_dir(){
                if args.len() >= 2{
                    let mut handle = fs::OpenOptions::new()
                        .write(true)
                        .read(true)
                        .create(true)
                        .open(homeDir.clone() + &args[0] + "/" + &args[0] + ".conf")
                        .unwrap();
        
                    for package in &args[1..]{
                        handle.rewind().unwrap();
                        let (mut text, mut uText) = (String::new(), String::new());
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
        
                                if packages.contains(&(package.to_owned() + "\n")){
                                    println!("{} Package ({}) already added to group ({})!", "[!]".yellow(), package.yellow(), args[0].yellow());
                                }
                                else{
                                    handle.rewind().unwrap();
                                    handle.write_all(
                                        (segments[0].to_owned()
                                        + "[PACKAGES]\n" 
                                        + &package
                                        + segments[1]
                                    ).as_bytes()).unwrap();
                                    println!("{} Added package ({}) to group ({})...", "[+]".green(), package.green(), args[0].green());
                                }
                            }
                            _ =>(),
                        }           
                    }
                }
                else{
                    println!("{} expected package(s)! (use -h for help)",
                        "error:".red()
                    );
                    std::process::exit(1);
                }
            }
            else{
                println!("{} group ({}) does not exist! (use -h for help)", 
                    "error:".red(), 
                    args[0].red()
                );
                std::process::exit(1);
            }
        }
        else{
            println!(
                "{} expected group name! (use -h for help)",
                "error:".red()
            );
            std::process::exit(1);
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
    let homeDir = dirs::home_dir()
        .unwrap()
        .into_os_string()
        .into_string()
        .unwrap() + "/.config/alps/";

    if flags.contains(&'h'){
        println!("usage: {{-S}} [options] [group]");
        println!("options:");
        println!("-g\tsync entire group configuration");
        println!("-p\tsync only group packages");
        println!("-f\tsync only group files");
    }
    else if flags.contains(&'p'){
        if Path::new(&(homeDir.clone() + &args[0])).is_dir(){
            let (mut packages, mut text) = (String::new(), String::new());
    
            let mut handle = fs::OpenOptions::new()
                .read(true)
                .open(homeDir.clone() + &args[0] + "/" + &args[0] + ".conf")
                .unwrap();
            handle.read_to_string(&mut text).unwrap();
            let mut text: Vec<&str> = text.split("[PACKAGES]").collect();
    
            for package in text[1]
                .split("[PATHS]")
                .next().unwrap()
                .split_whitespace(){
                    let handle = Command::new("pacman")
                        .args(["-Q"])
                        .arg(&package)
                        .output()
                        .expect("Failed to run command.");
            
                    if !handle.status.success(){
                        println!("{} Added package ({}) to install...", "[+]".green(), package.green());
                        packages += &(package.to_owned() + " ");
                    }
                    else{
                        println!("{} Package ({}) already exists!", "[!]".yellow(), package.yellow());
                    }
            }

            if !packages.is_empty(){
                let mut handle = Command::new("sudo");
                let mut command = handle.args(["pacman", "-S"]);
        
                for substr in packages.as_str().split_whitespace(){
                    command = command.arg(substr);
                }
        
                command.status().expect("Failed to run command");
            }
        }
        else{
            println!("{} Group ({}) does not exist! (use -h for help)", "[!]".yellow(), args[0].yellow());
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
                            println!("{} option ({}) not found! (use -h for help)",
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