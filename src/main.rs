use std::{
    fs,
    env, 
    path::Path, 
    process::Command,
    collections::HashSet, 
    io::{prelude::*, ErrorKind, self},
}; 
use colored::Colorize;
use fs_extra::dir::{self, CopyOptions};

fn initGroup(group: String, home_dir: String){
    fs::create_dir(home_dir.clone() + &group + "/configs");
    fs::File::create(home_dir + &group + "/configs/" + &group + ".confg");;
}

fn configAdd(home_dir: String, label: &str, args: Vec<String>, excludes: Vec<String>){
    let mut handle = fs::OpenOptions::new()
        .write(true)
        .read(true)
        .create(true)
        .open(home_dir.clone() + &args[0] + "/" + &args[0] + ".conf")
        .unwrap();
    
    for package in &args[1..]{
        handle.rewind().unwrap();
        let (mut text, mut u_text) = (String::new(), String::new());
        handle.read_to_string(&mut text).unwrap();
    
        let mut segments: Vec<&str> = text.split(label).collect();
        if segments.len() < 2{
            handle.seek(std::io::SeekFrom::End(0)).unwrap();
            handle.write_all(label.as_bytes()).unwrap();
        }
        handle.rewind().unwrap();
        handle.read_to_string(&mut u_text).unwrap();
        segments = u_text.split(label).collect();
    
        match segments.len(){
            3.. => panic!("Redefinition of label {}", label),
            2 =>{
                let mut packages = segments[1];
                for exclude in &excludes{
                    packages = packages.split(exclude).next().unwrap();
                }
                
                let mut contains = false;
                for has in packages.split_whitespace(){
                    if package == has{
                        println!(
                            "{} Package ({}) already added to group ({})!", 
                            "[!]".yellow(), 
                            package.yellow(), 
                            args[0].yellow()
                        );
                        contains = true;
                        break;
                    }
                }
                if !contains{
                    handle.rewind().unwrap();
                    handle.write_all(
                        (segments[0].to_owned()
                        + label + "\n" 
                        + package
                        + segments[1]
                    ).as_bytes()).unwrap();
                    println!(
                        "{} Added package ({}) to group ({})...", 
                        "[+]".green(), 
                        package.green(), 
                        args[0].green()
                    );
                }
            }
            _ =>(),
        }           
    }
}

fn install(flags: HashSet<char>, args: Vec<String>){
    let excludes = vec![
        String::from("[PACKAGES"),
        String::from("[PATHS]"),
        String::from("[SCRIPTS]")
    ];
    let home_dir = dirs::home_dir()
        .unwrap()
        .into_os_string()
        .into_string()
        .unwrap() + "/.config/alps/";

    fs::create_dir(home_dir.clone());

    if flags.contains(&'h'){
        println!("usage: {{-I}} [options] [package(s)]");
        println!("options:");
        println!("-f\tinstall file to profile");
        println!("-p\tinstall package to profile")
    } 
    else if flags.contains(&'g'){
        for arg in args{
            match fs::create_dir(home_dir.clone() + &arg){
                Ok(()) => println!(
                    "{} Created group ({})...", 
                    "[+]".green(), 
                    arg.green()
                ),
                Err(error) =>{
                    if error.kind() == ErrorKind::AlreadyExists{
                        println!(
                            "{} Group ({}) already exists!", 
                            "[!]".yellow(), 
                            arg.yellow()
                        );
                    }
                }
            }
        }
    }
    else if flags.contains(&'p'){ 
        if !args.is_empty(){
            if Path::new(&(home_dir.clone() + &args[0])).is_dir(){
                if args.len() >= 2{
                    configAdd(home_dir,"[PACKAGES]", args, excludes);
                }
                else{
                    println!(
                        "{} expected package(s)! (use -h for help)",
                        "error:".red()
                    );
                    std::process::exit(1);
                }
            }
            else{
                println!(
                    "{} group ({}) does not exist! (use -h for help)", 
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
    else if flags.contains(&'f'){
        if !args.is_empty(){
            if Path::new(&(home_dir.clone() + &args[0])).is_dir(){
                let mut paths: Vec<String> = vec![args[0].clone()];

                for arg in &args[1..]{
                    match fs::canonicalize(std::path::PathBuf::from(arg)){
                        Ok(path)=>{
                            paths.push(
                                path.into_os_string()
                                .into_string()
                                .unwrap()
                            );
                        }
                        Err(_)=> println!(
                            "{} path does not exist!",
                            "error:".red()
                        ),
                    }
                }

                let mut options = CopyOptions::new();
                options.copy_inside = true;
                options.overwrite = true;
                
                fs::create_dir(home_dir.clone() + &args[0] + "/configs");
                for path in &paths[1..]{
                    let md = fs::metadata(path).unwrap();

                    if md.is_dir(){
                        dir::copy(path, home_dir.clone() + &args[0] + "/configs", &options).unwrap();
                    }
                    else if md.is_file(){
                        fs::copy(path, home_dir.clone() + &args[0] + "/configs/" + path.split("/").last().unwrap()).unwrap();
                    }
                }

                configAdd(home_dir, "[PATHS]", paths, excludes);
            }
            else{
                println!(
                    "{} group ({}) does not exist! (use -h for help)",
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
    let excludes = vec![
        String::from("[PACKAGES"),
        String::from("[PATHS]"),
        String::from("[SCRIPTS]")
    ];
    let home_dir = dirs::home_dir()
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
        if args.is_empty(){
            if Path::new(&(home_dir.clone() + &args[0])).is_dir(){
                let (mut packages, mut text) = (String::new(), String::new());
        
                let mut handle = fs::OpenOptions::new()
                    .read(true)
                    .open(home_dir.clone() + &args[0] + "/" + &args[0] + ".conf")
                    .unwrap();
                handle.read_to_string(&mut text).unwrap();
                let text: Vec<&str> = text.split("[PACKAGES]").collect();

                if text.len() > 1{
                    let mut text = text[1];
    
                    for exclude in &excludes{
                        text = text.split(exclude).next().unwrap();
                    }
                    
                    for package in text.split_whitespace(){
                        let handle = Command::new("pacman")
                            .args(["-Q"])
                            .arg(&package)
                            .output()
                            .expect("Failed to run command.");
                
                        if !handle.status.success(){
                            println!(
                                "{} Added package ({}) to install...", 
                                "[+]".green(), 
                                package.green()
                            );
                            packages += &(package.to_owned() + " ");
                        }
                        else{
                            println!(
                                "{} Package ({}) already exists!", 
                                "[!]".yellow(), 
                                package.yellow()
                            );
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
            }
            else{
                println!(
                    "{} Group ({}) does not exist! (use -h for help)", 
                    "[!]".yellow(), 
                    args[0].yellow()
                );
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
    else if flags.contains(&'f'){
        if !args.is_empty(){
            let (mut packages, mut text) = (String::new(), String::new());
    
            let mut handle = fs::OpenOptions::new()
                .read(true)
                .open(home_dir.clone() + &args[0] + "/" + &args[0] + ".conf")
                .unwrap();
            handle.read_to_string(&mut text).unwrap();
            let text: Vec<&str> = text.split("[PATHS]").collect();           

            if text.len() > 1{
                let mut text = text[1];

                for exclude in &excludes{
                    text = text.split(exclude).next().unwrap();
                }

                for package in text.split_whitespace(){
                    println!("{package}");
                }

            }
        }
        else{
            println!(
                "{} expected group name! (user -h for help)",
                "error:".red()
            );
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
                            println!(
                                "{} option ({}) not found! (use -h for help)",
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