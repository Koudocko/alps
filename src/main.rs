use std::{
    fs,
    env, 
    path::Path, 
    process::Command,
    collections::HashSet, 
    io::{prelude::*, ErrorKind},
}; 
use colored::Colorize;
use fs_extra::dir::{self, CopyOptions};

enum Exception<'a, 'b>{
    MissingContent(&'a str),
    InvalidGroup(&'b str),
    MissingGroup,
}

impl<'a, 'b> Exception<'a, 'b>{
    fn handle(&self){
        match self{
            Exception::InvalidGroup(group)=>{
                eprintln!(
                    "{} Group ({}) does not exist! (use -h for help)", 
                    "[!!!]".red(), 
                    group.red()
                );
            }
            Exception::MissingContent(label)=>{
                println!(
                    "{} Label ({}) does not have content! (use -h for help)",
                    "[!!!]".red(),
                    label.red()
                );
            }
            Exception::MissingGroup=>{
                eprintln!(
                    "{} Expected group name! (use -h for help)",
                    "[!!!]".red()
                );
                std::process::exit(1);
            }
        }       
    }
}

fn read_label<'a, 'b>(label: &'a str, group:  &'b str)-> Result<String, Exception<'a, 'b>>{
    let excludes = vec![
        String::from("[PACKAGES]"),
        String::from("[PATHS]"),
        String::from("[SCRIPTS]")
    ];
    let home_dir = dirs::home_dir()
        .unwrap()
        .into_os_string()
        .into_string()
        .unwrap() + "/.config/alps/";

    if !group.is_empty(){
        if Path::new(&(home_dir.clone() + group)).is_dir(){
            let mut text = String::new();
    
            let mut handle = fs::OpenOptions::new()
                .read(true)
                .write(true)
                .create(true)
                .open(home_dir + group + "/" + group + ".conf")
                .unwrap();
            handle.read_to_string(&mut text).unwrap();
            let text: Vec<&str> = text.split(&label).collect();
    
            if text.len() > 1{
                let mut text = text[1];
    
                for exclude in &excludes{
                    text = text.split(exclude).next().unwrap();
                }
    
                return Ok(text.to_string());
            }
            else{
                return Err(Exception::MissingContent(label));
            }
        }
        else{
            return Err(Exception::InvalidGroup(group));
        }
    }
    
    Err(Exception::MissingGroup)
}

fn config_add(home_dir: String, label: &str, args: Vec<String>){
    let excludes = vec![
        String::from("[PACKAGES]"),
        String::from("[PATHS]"),
        String::from("[SCRIPTS]")
    ];

    for arg in &args[1..]{
        let mut contains = false;

        if let Ok(text) = read_label(label, &args[0]){
            for entry in text.split_whitespace(){
                if arg == entry{
                    println!(
                        "{} Entry ({}) already added to group ({}) under label ({})!", 
                        "[!]".yellow(), 
                        entry.yellow(), 
                        args[0].yellow(),
                        label.yellow()
                    );               
                    
                    contains = true;
                    break;
                }
            }
        }

        if !contains{
            println!(
                "{} Added entry ({}) to group ({}) under label ({})...", 
                "[+]".green(), 
                arg.green(), 
                args[0].green(),
                label.green()
            );

            let mut handle = fs::OpenOptions::new()
                .write(true)
                .open(home_dir.clone() + &args[0] + "/" + &args[0] + ".conf")
                .unwrap();

            let mut segments: Vec<String> = Vec::new();
            for segment in &excludes{
                let list = if let 
                    Ok(list) = read_label(segment, &args[0]){list}
                else{String::new()};

                if segment == label{
                    segments.push(segment.to_owned() + "\n" + arg + &list);
                }
                else{
                    segments.push(segment.to_owned() + &list);
                }
            }
            
            for mut segment in segments{
                if !segment.ends_with('\n'){
                    segment.push('\n');
                }
                handle.write_all(segment.as_bytes()).unwrap();
            }
        }
    }
}

fn install(flags: HashSet<char>, args: Vec<String>){
    let home_dir = dirs::home_dir()
        .unwrap()
        .into_os_string()
        .into_string()
        .unwrap() + "/.config/alps/";

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
                    config_add(home_dir,"[PACKAGES]", args);
                }
                else{
                    eprintln!(
                        "{} Expected package(s)! (use -h for help)",
                        "[!!!]".red()
                    );
                    std::process::exit(1);
                }
            }
            else{
                eprintln!(
                    "{} Group ({}) does not exist! (use -h for help)", 
                    "[!!!]".red(), 
                    args[0].red()
                );
                std::process::exit(1);
            }
        }
        else{
            eprintln!(
                "{} Expected group name! (use -h for help)",
                "[!!!]".red()
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
                        Err(_)=> eprintln!(
                            "{} Path to ({}) does not exist!",
                            "[!!!]".red(),
                            arg
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

                config_add(home_dir, "[PATHS]", paths);
            }
            else{
                eprintln!(
                    "{} Group ({}) does not exist! (use -h for help)",
                    "[!!!]".red(),
                    args[0].red()
                );
                std::process::exit(1);
            }
        }
        else{
            eprintln!(
                "{} Expected group name! (use -h for help)",
                "[!!!]".red()
            );
            std::process::exit(1);
        }
    }
    else{
        eprintln!(
            "{} Invalid option! (use -h for help)",
            "[!!!]".red()
         );
         std::process::exit(1);
    }
}

fn remove(flags: HashSet<char>, args: Vec<String>){
    
}

fn sync(flags: HashSet<char>, args: Vec<String>){
    if flags.contains(&'h'){
        println!("usage: {{-S}} [options] [group]");
        println!("options:");
        println!("-g\tsync entire group configuration");
        println!("-p\tsync only group packages");
        println!("-f\tsync only group files");
    }
    else if flags.contains(&'p'){
        match read_label("[PACKAGES]", &args
            .into_iter()
            .next()
            .unwrap_or_default()
            ){
            Ok(text)=>{
                let mut packages = String::new(); 
                   
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
            Err(error)=> error.handle(),
        }
    }
    else if flags.contains(&'f'){
        match read_label("[PATHS]", &args
            .into_iter()
            .next()
            .unwrap_or_default()
            ){
            Ok(text)=>{
                for path in text.split_whitespace(){
                    let mut options = CopyOptions::new();
                    options.copy_inside = true;
                    options.overwrite = true;
                    options.content_only = true;
                    
                    let md = fs::metadata(path).unwrap();
                    if md.is_dir(){
                        print!("eek");
                        //dir::copy(home_dir.clone() + &args[0] + "/configs", path, &options).unwrap();
                    }
                    else if md.is_file(){
                        //fs::copy(home_dir.clone() + &args[0] + "/configs/" + path.split("/").last().unwrap(), path).unwrap();
                    }
                }
            }
            Err(error)=> error.handle(),
        }
    }
    else{
        eprintln!(
            "{} Invalid option! (use -h for help)",
            "[!!!]".red()
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
        
    }
    else{
        eprintln!(
            "{} Invalid option! (use -h for help)",
            "[!!!]".red()
         );
         std::process::exit(1);
    }
}


fn parser(){
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
                                eprintln!(
                                    "{} Only one operation may be used at a time",
                                    "[!!!]".red()
                                );
                                std::process::exit(1);
                            }
                        }
                        'h' | 'f' | 'p' | 'c' | 'g' => {
                            flags.insert(flag);
                        }
                        op =>{ 
                            eprintln!(
                                "{} Option ({}) not found! (use -h for help)",
                                "[!!!]".red(),
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

fn main(){
    parser();
}