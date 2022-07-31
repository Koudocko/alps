use std::{
    fs,
    env, 
    path::Path, 
    process::Command,
    collections::HashSet, 
    io::{prelude::*, ErrorKind}, 
}; 
use colored::Colorize;

enum Exception{
    InvalidOperation(char),
    InvalidGroup(String),
    MissingArgs(String),
    Status(i32),
    DuplicateOperation,
    MissingOperation,
    InvalidOption,
    MissingOption,
    MissingGroup,
    PlainRun,
}

impl Exception{
    fn handle(&self){
        let mut exit_status = 1;

        match self{
            Exception::MissingArgs(label) =>{
                eprintln!(
                    "{} Expected {}(s)! (use -h for help)",
                    "[!!!]".red(),
                    label.red()
                )
            }
            Exception::InvalidGroup(group) =>{
                eprintln!(
                    "{} Group ({}) does not exist! (use -h for help)", 
                    "[!!!]".red(), 
                    group.red()
                );
            }
            Exception::MissingGroup =>{
                eprintln!(
                    "{} Expected group name! (use -h for help)",
                    "[!!!]".red()
                );
            }
            Exception::DuplicateOperation =>{
                eprintln!(
                    "{} Only one operation may be used at a time! (use -h for help)",
                    "[!!!]".red()
                );
            }
            Exception::InvalidOperation(op) =>{
                eprintln!(
                    "{} Option ({}) not found! (use -h for help)",
                    "[!!!]".red(),
                    op
                );
            }
            Exception::PlainRun =>{
                eprintln!(
                    "{} No operation specified! (use -h for help)",
                    "[!!!]".red()
                )
            }
            Exception::InvalidOption =>{
                eprintln!(
                    "{} Invalid option! (use -h for help)",
                    "[!!!]".red()
                );
            }
            Exception::Status(status) =>{
                exit_status = *status;
            }
            Exception::MissingOperation =>{
                eprintln!(
                    "{} Missing operation! (use -h for help)",
                    "[!!!]".red()
                )
            }
            Exception::MissingOption =>{
                eprintln!(
                    "{} Missing option! (use -h for help)",
                    "[!!!]".red()
                )
            }
        }       
        
        std::process::exit(exit_status);
    }
}

fn copy_dir<S, D>(src: S, dst: D)
where
   S: AsRef<Path>,
   D: AsRef<Path>,
{
   let path = Path::new(src.as_ref());

   if path.is_dir(){
      fs::create_dir_all(&dst).unwrap();

      for dir in fs::read_dir(src).unwrap(){
         let dir = dir.unwrap();
         copy_dir(dir.path(), dst.as_ref().join(dir.file_name()));
      }
   }
   else if path.is_file(){
      fs::copy(src, dst).unwrap();
   }
}

fn read_label(label: &str, group:  &str)-> Result<String, Exception>{
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
    
                Ok(text.to_string())
            }
            else{
                Ok(String::new())
            }
        }
        else{
            Err(Exception::InvalidGroup(group.to_string()))
        }
    }
    else{
        Err(Exception::MissingGroup)
    }
}

fn config_add(home_dir: String, label: &str, args: Vec<String>, mode: i32)-> Result<(), Exception>{
    if let Err(error) = read_label(label, args.get(0).unwrap_or(&String::new())){
        return Err(error);
    }

    if args.len() < 2{
        return Err(Exception::MissingArgs(label.to_string()));
    }

    let excludes = vec![
        String::from("[PACKAGES]"),
        String::from("[PATHS]"),
        String::from("[SCRIPTS]")
    ];

    for arg in &args[1..]{
        let mut contains = false;

        let handle = read_label(label, &args[0]);
        match handle{
            Ok(text) =>{
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
            Err(error) => return Err(error),
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

            if mode == 1{
                fs::create_dir(home_dir.clone() + &args[0] + "/configs");

                for path in &args[1..]{
                    copy_dir(path, home_dir.clone() + &args[0] + "/configs/" + path.split('/').last().unwrap());
                }
            }
        }
    }

    Ok(())
}

fn install(flags: HashSet<char>, args: Vec<String>)-> Result<(), Exception>{
    let home_dir = dirs::home_dir()
        .unwrap()
        .into_os_string()
        .into_string()
        .unwrap() + "/.config/alps/";

    if flags.contains(&'h'){
        println!("usage: {{-I}} [options] [package(s)]");
        println!("options:");
        println!("-g\tinstall group to config");
        println!("-f\tinstall file to profile");
        println!("-p\tinstall package to profile");
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
        return config_add(home_dir.clone(), "[PACKAGES]", args, 0);
    }
    else if flags.contains(&'f'){
        let handle = read_label("[PATHS]", args.get(0).unwrap_or(&String::new()));

        match handle{
            Ok(_) =>{
                let mut paths: Vec<String> = vec![args[0].clone()];
        
                for arg in &args[1..]{
                    match fs::canonicalize(std::path::PathBuf::from(arg)){
                        Ok(path) =>{
                            paths.push(
                                path.into_os_string()
                                .into_string()
                                .unwrap()
                            );
                        }
                        Err(_) => eprintln!(
                            "{} Path to ({}) does not exist!",
                            "[!!!]".red(),
                            arg
                        ),
                    }
                }
        
                return config_add(home_dir, "[PATHS]", paths, 1);
            }
            Err(error) => return Err(error),
        }
    }
    else{
        if flags.is_empty(){
            return Err(Exception::MissingOption);
        }

        return Err(Exception::InvalidOption);
    }

    Ok(())
}

fn remove(flags: HashSet<char>, args: Vec<String>)-> Result<(), Exception>{
    Ok(())
}

fn sync(flags: HashSet<char>, args: Vec<String>)-> Result<(), Exception>{
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
        match read_label("[PACKAGES]", args
            .get(0)
            .unwrap_or(&String::new())
            ){
            Ok(text) =>{
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
            Err(error) => return Err(error),
        }
    }
    else if flags.contains(&'f'){
        match read_label("[PATHS]", args
            .get(0)
            .unwrap_or(&String::new())
            ){
            Ok(text) =>{
                for path in text.split_whitespace(){
                    let path_name = path.split('/').last().unwrap();
                        copy_dir(home_dir.clone() + &args[0] + "/configs/" + path_name, path);

                    println!(
                        "{} Synced ({}) successfully!",
                        "[~]".purple(),
                        path_name,
                    )
                }
            }
            Err(error) => return Err(error),
        }
    }
    else{
        if flags.is_empty(){
            return Err(Exception::MissingOption);
        }

        return Err(Exception::InvalidOption);
    }
    Ok(())
}

fn query(flags: HashSet<char>, args: Vec<String>)-> Result<(), Exception>{
    let home_dir = dirs::home_dir()
        .unwrap()
        .into_os_string()
        .into_string()
        .unwrap() + "/.config/alps/";

    if flags.contains(&'h'){
        println!("usage: {{-Q}} [options] [?group] [optional:entries]"); 
        println!("options:");
        println!("-g\tlist all groups installed to configs, including package and script totals");
        println!("-p\tlist all group packages");
        println!("-f\tlist all group files");
    }
    else if flags.contains(&'g'){
        //Credit Raforawesome (aka God)
        let groups: Vec<fs::DirEntry> = fs::read_dir(home_dir).unwrap()
            .filter(|entry|{
                let entry = entry.as_ref().unwrap()
                    .file_type().unwrap();

                entry.is_dir()
            }).map(|x| x.unwrap()).collect();

        if !args.is_empty(){
            let mut status = 0;

            for arg in &args{
                let mut contains = false;
                for group in &groups{
                    let group = &group.file_name().into_string().unwrap();

                    if arg == group{
                        println!(
                            "{} Found group ({})...", 
                            "[?]".blue(),
                            group.blue()
                        );                   
    
                        contains = true;
                        break;
                    }
                }

                if !contains{
                    eprintln!(
                        "{} Group ({}) not found!",
                        "[!]".yellow(),
                        arg.yellow()
                    );
                    status += 1;
                }               
            }

            return Err(Exception::Status(status));
        }
        else{
            for group in &groups{
                let group = group.file_name().into_string().unwrap();

                println!(
                    "{} :: ({}) packages :: ({}) configs :: ({}) scripts", 
                    group.blue(),
                    if let Ok(text) = read_label("[PACKAGES]", &group){text.split_whitespace().count()}
                        else{0},
                    if let Ok(text) = read_label("[PATHS]", &group){text.split_whitespace().count()}
                        else{0},
                    if let Ok(text) = read_label("[SCRIPTS]", &group){text.split_whitespace().count()}
                        else{0}
                );
            }
            println!("({}) groups found...", groups.len());
        }
    }
    else if flags.contains(&'p'){
        match read_label("[PACKAGES]", args
            .get(0)
            .unwrap_or(&String::new())
        ){
            Ok(text) =>{
                if args.len() > 1{
                    let mut status = 0;

                    for arg in &args[1..]{
                        let mut contains = false;
                        for package in text.split_whitespace(){
                            if arg == package{
                                println!(
                                    "{} Found package ({}) in group ({})...", 
                                    "[?]".blue(),
                                    package.blue(),
                                    &args[0].blue()
                                );
                                
                                contains = true;
                                break;
                            }
                        }

                        if !contains{
                            eprintln!(
                                "{} Package ({}) not found in group ({})!",
                                "[!]".yellow(),
                                arg.yellow(),
                                &args[0].yellow()
                            );
                            status += 1;
                        }
                    }

                    return Err(Exception::Status(status));
                }
                else{
                    let mut count = 0;
                    for package in text.split_whitespace(){
                        print!("{}, ", package.blue());
                        count += 1;
                    }
                    println!("({count}) packages found...");
                }
            }
            Err(error) => error.handle(),
        }
    }
    else if flags.contains(&'f'){
        match read_label("[PATHS]", args
            .get(0)
            .unwrap_or(&String::new())
        ){
            Ok(text) =>{
                if args.len() > 1{
                    let mut status = 0;

                    for arg in &args[1..]{
                        let mut contains = false;
                        for path in text.split_whitespace(){
                            if arg == path.split('/').last().unwrap(){
                                println!(
                                    "{} Found config ({}) in group ({})...", 
                                    "[?]".blue(),
                                    arg.blue(),
                                    &args[0].blue()
                                );
                                
                                contains = true;
                                break;
                            }
                        }

                        if !contains{
                            eprintln!(
                                "{} Config ({}) not found in group ({})!",
                                "[!]".yellow(),
                                arg.yellow(),
                                &args[0].yellow()
                            );
                            status += 1;
                        }
                    }

                    return Err(Exception::Status(status));
                }
                else{
                    let mut count = 0;
                    for  path in text.split_whitespace(){
                        print!("{}, ", path.split('/').last().unwrap().blue());
                        count += 1;
                    }
                    println!("({count}) configs found...");
                }
            }
            Err(error) => return Err(error),
        }
    }
    else{
        if flags.is_empty(){
            return Err(Exception::MissingOption);
        }

        return Err(Exception::InvalidOption);
    }

    Ok(())
}

fn parser()-> Result<(), Exception>{
    fs::create_dir(dirs::home_dir()
        .unwrap()
        .into_os_string()
        .into_string()
        .unwrap() + "/.config/alps/"
    );

    let p_args: Vec<_> = env::args().collect();
    if p_args.len() == 1{ return Err(Exception::PlainRun); }

    let mut args: Vec<String> = Vec::new();
    let mut flags = HashSet::new();
    let mut mode = None; 

    for arg in &p_args[1..]{
        if arg.as_bytes()[0] as char == '-'{
            for flag in arg.chars().skip(1){
                match flag{
                    'I' | 'R' | 'S' | 'Q' =>{
                        if mode == None{
                            mode = Some(flag); 
                        }
                        else{
                            return Err(Exception::DuplicateOperation);
                        }
                    }
                    'h' | 'f' | 'p' | 'c' | 'g' => {
                        flags.insert(flag);
                    }
                    op =>{ 
                        return Err(Exception::InvalidOperation(op));
                    }
                }
            }
            continue;
        }
        args.push(arg.clone());
    }

    match mode{
        Some('I') => install(flags, args), 
        Some('R') => remove(flags, args),
        Some('S') => sync(flags, args),
        Some('Q') => query(flags, args),
        None => Err(Exception::MissingOperation),
        _ => Ok(()),
    }
}

fn main(){
    if let Err(error) = parser(){
        error.handle();
    }
}