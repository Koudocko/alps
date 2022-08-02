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

fn reformat_config(labels: &Vec<String>, group: &str, home_dir: &str){
    let mut config_text = String::new();

    for label in labels{
        config_text.push_str(&(label.to_owned() + "\n"));

        if let Ok(text) = read_label(label, group, home_dir){
            for entry in text.split_whitespace(){
                config_text.push_str (&(entry.to_owned() + "\n"));
            }
        }

        if label != labels.last().unwrap(){
            config_text.push('\n');
        }
    }
    config_text.push('\n');

    let mut handle = fs::OpenOptions::new()
        .truncate(true)
        .write(true)
        .open(home_dir.to_owned() + group + "/" + group + ".conf")
        .unwrap();

    handle.write_all(config_text.as_bytes()).unwrap();
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

fn read_label(label: &str, group:  &str, home_dir: &str)-> Result<String, Exception>{
    let excludes = vec![
        String::from("[PACKAGES]"),
        String::from("[PATHS]"),
        String::from("[SCRIPTS]")
    ];

    if !group.is_empty(){
        if Path::new(&(home_dir.to_owned() + group)).is_dir(){
            let mut text = String::new();
    
            let mut handle = fs::OpenOptions::new()
                .read(true)
                .write(true)
                .create(true)
                .open(home_dir.to_owned() + group + "/" + group + ".conf")
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

fn config_del(label: &str, args: Vec<String>, home_dir: &str)-> Result<(), Exception>{
    if let Err(error) = read_label(label, args.get(0).unwrap_or(&String::new()), home_dir){
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

    reformat_config(&excludes, &args[0], home_dir);

    for arg in &args[1..]{
        let mut contains = false;

        let readin = read_label(label, &args[0], home_dir);
        match readin{
            Ok(text) =>{
                for entry in text.split_whitespace(){
                    if arg == entry{
                        println!(
                            "{} Removing entry ({}) from group ({}) under label ({})!", 
                            "[-]".green(), 
                            entry.green(), 
                            args[0].green(),
                            label.green()
                        );               
                        
                        contains = true;
                        break;
                    }
                }
            }
            Err(error) => return Err(error),
        }

        if contains{
            let mut segments: Vec<String> = Vec::new();
            for segment in &excludes{
                let list = if let 
                    Ok(list) = read_label(segment, &args[0], home_dir){list.split(arg).collect::<String>()}
                else{String::new()};

                if segment == label{
                    segments.push(segment.to_owned() + "\n" + &list);
                }
                else{
                    segments.push(segment.to_owned() + &list);
                }
            }

            let mut handle = fs::OpenOptions::new()
                .truncate(true)
                .write(true)
                .open(home_dir.to_owned() + &args[0] + "/" + &args[0] + ".conf")
                .unwrap();           

            for mut segment in segments{
                if !segment.ends_with('\n'){
                    segment.push('\n');
                }

                handle.write_all(segment.as_bytes()).unwrap();
            }
        }
        else{
            eprintln!(
                "{} Entry ({}) not found under group ({}) label ({})",
                "[!]".yellow(),
                arg.yellow(),
                args[0].yellow(),
                label.yellow()
            )
        }
    }

    reformat_config(&excludes, &args[0], home_dir);
    Ok(())
}

fn config_add(label: &str, args: Vec<String>, home_dir: &str)-> Result<(), Exception>{
    if let Err(error) = read_label(label, args.get(0).unwrap_or(&String::new()), home_dir){
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

    reformat_config(&excludes, &args[0], home_dir);

    for arg in &args[1..]{
        let mut contains = false;

        let readin = read_label(label, &args[0], home_dir);
        match readin{
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

            let mut segments: Vec<String> = Vec::new();
            for segment in &excludes{
                let list = if let 
                    Ok(list) = read_label(segment, &args[0], home_dir){list}
                else{String::new()};

                if segment == label{
                    segments.push(segment.to_owned() + "\n" + arg + &list);
                }
                else{
                    segments.push(segment.to_owned() + &list);
                }
            }

            let mut handle = fs::OpenOptions::new()
                .truncate(true)
                .write(true)
                .open(home_dir.to_owned() + &args[0] + "/" + &args[0] + ".conf")
                .unwrap();                 

            for mut segment in segments{
                if !segment.ends_with('\n'){
                    segment.push('\n');
                }
                handle.write_all(segment.as_bytes()).unwrap();
            }
        }
    }

    reformat_config(&excludes, &args[0], home_dir);
    Ok(())
}

fn install(flags: HashSet<char>, args: Vec<String>, home_dir: &str)-> Result<(), Exception>{
    if flags.contains(&'h'){
        println!("usage: {{-I}} [options] [package(s)]");
        println!("options:");
        println!("-g\tinstall group to config");
        println!("-f\tinstall file to profile");
        println!("-p\tinstall package to profile");
    } 
    else if flags.contains(&'g'){
        for arg in args{
            match fs::create_dir(home_dir.to_owned() + &arg){
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
        return config_add("[PACKAGES]", args, home_dir);
    }
    else if flags.contains(&'f'){
        fs::create_dir(home_dir.to_owned() + &args[0] + "/configs");
        let handle = read_label("[PATHS]", args.get(0).unwrap_or(&String::new()), home_dir);

        match handle{
            Ok(_) =>{
                let mut paths: Vec<String> = vec![args[0].clone()];
        
                for arg in &args[1..]{
                    match fs::canonicalize(std::path::PathBuf::from(arg)){
                        Ok(path) =>{
                            let path_str = path.clone().into_os_string().into_string().unwrap();

                            paths.push(path_str.clone());

                            let contains = fs::read_dir(home_dir.to_owned()
                                + &args[0] 
                                + "/configs/").unwrap()
                            .map(|x|{
                                x.unwrap().file_name().into_string().unwrap()}
                            ).any(|group| group == *arg);

                            if !contains{
                                copy_dir(
                                path, 
                                home_dir.to_owned() + 
                                    &args[0] + "/configs/" + 
                                    path_str
                                    .split('/')
                                    .last()
                                    .unwrap()
                                );
                            }
                        }
                        Err(_) => println!(
                            "{} Path to ({}) does not exist!",
                            "[!]".yellow(),
                            arg.yellow()
                        ),
                    }
                }

                return config_add("[PATHS]", paths, home_dir);
            }
            Err(error) => return Err(error),
        }
    }
    else if flags.contains(&'s'){
        fs::create_dir(home_dir.to_owned() + &args[0] + "/scripts");
        let handle = read_label("[SCRIPTS]", args.get(0).unwrap_or(&String::new()), home_dir);

        match handle{
            Ok(_) =>{
                for arg in &args[1..]{
                    match fs::canonicalize(std::path::PathBuf::from(arg)){
                        Ok(path) =>{
                            let path_str = path.clone().into_os_string().into_string().unwrap();

                            let contains = fs::read_dir(home_dir.to_owned() 
                                + &args[0] 
                                + "/configs/").unwrap()
                            .map(|x|{
                                x.unwrap().file_name().into_string().unwrap()}
                            ).any(|group| group == *arg);


                            if !contains{
                                copy_dir(
                                path, 
                                home_dir.to_owned() + 
                                    &args[0] + "/scripts/" + 
                                    path_str
                                    .split('/')
                                    .last()
                                    .unwrap()
                                );
                            }
                        }
                        Err(_) => println!(
                            "{} Path to ({}) does not exist!",
                            "[!]".yellow(),
                            arg.yellow()
                        ),
                    }
                }
                
                return config_add("[SCRIPTS]", args, home_dir);
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

fn remove(flags: HashSet<char>, args: Vec<String>, home_dir: &str)-> Result<(), Exception>{
    if flags.contains(&'h'){
        println!("usage: {{-R}} [options] [group]");
        println!("options:");
        println!("-g\tsync entire group configuration");
        println!("-p\tsync only group packages");
        println!("-f\tsync only group files");       
    }
    else if flags.contains(&'p'){
        return config_del("[PACKAGES]", args, home_dir);
    }
    else{
        if flags.is_empty(){
            return Err(Exception::MissingOption);
        }

        return Err(Exception::InvalidOption);
    }

    Ok(())
}

fn sync(flags: HashSet<char>, args: Vec<String>, home_dir: &str)-> Result<(), Exception>{
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
            .unwrap_or(&String::new()),
            home_dir
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
            

                    match command.status(){
                        Ok(_) =>{
                            println!(
                                "{} Successfully synced packages!",
                                "[~]".purple()
                            )
                        }
                        Err(_) =>{
                            eprintln!(
                                "{} Failed to sync packages!",
                                "[!]".yellow()
                            );
                        }
                    }
                }
            }
            Err(error) => return Err(error),
        }
    }
    else if flags.contains(&'f'){
        match read_label("[PATHS]", args
            .get(0)
            .unwrap_or(&String::new()),
            home_dir
            ){
            Ok(text) =>{
                for path in text.split_whitespace(){
                    let path_name = path.split('/').last().unwrap();
                        copy_dir(home_dir.to_owned() + &args[0] + "/configs/" + path_name, path);

                    println!(
                        "{} Synced ({}) successfully!",
                        "[~]".purple(),
                        path_name.purple(),
                    )
                }
            }
            Err(error) => return Err(error),
        }
    }
    else if flags.contains(&'s'){
        match read_label("[SCRIPTS]", args
            .get(0)
            .unwrap_or(&String::new()),
            home_dir
            ){
                Ok(text) =>{
                    for script in text.split_whitespace(){
                        let mut handle = Command::new("/".to_owned() + home_dir + &args[0] + "/scripts/" + script);
                        match handle.status(){
                            Ok(_) => println!(
                                "{} Successfully ran script ({})...",
                                "[~]".purple(),
                                script.purple()
                            ),
                            Err(error) => {
                                if error.kind() == ErrorKind::NotFound{
                                    println!(
                                        "{} Script ({}) not installed to group! (use -h for help)",
                                        "[!]".yellow(),
                                        script.yellow()
                                    )
                                }
                                else{
                                    println!(
                                        "{} Script ({}) failed to exit successfully!",
                                        "[!]".yellow(),
                                        script.yellow()
                                    )
                                }
                            }
                        }
                    }
                }
                Err(error) => error.handle(),
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

fn query(flags: HashSet<char>, args: Vec<String>, home_dir: &str)-> Result<(), Exception>{
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
                    if let Ok(text) = read_label("[PACKAGES]", &group, home_dir){text.split_whitespace().count()}
                        else{0},
                    if let Ok(text) = read_label("[PATHS]", &group, home_dir){text.split_whitespace().count()}
                        else{0},
                    if let Ok(text) = read_label("[SCRIPTS]", &group, home_dir){text.split_whitespace().count()}
                        else{0}
                );
            }
            println!("({}) groups found...", groups.len());
        }
    }
    else if flags.contains(&'p'){
        match read_label("[PACKAGES]", args
            .get(0)
            .unwrap_or(&String::new()),
            home_dir
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
            .unwrap_or(&String::new()),
            home_dir
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
    else if flags.contains(&'s'){
        match read_label("[SCRIPTS]", args
            .get(0)
            .unwrap_or(&String::new()),
            home_dir
        ){
            Ok(text) =>{
                if args.len() > 1{
                    let mut status = 0;

                    for arg in &args[1..]{
                        let mut contains = false;
                        for package in text.split_whitespace(){
                            if arg == package{
                                println!(
                                    "{} Found script ({}) in group ({})...", 
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
                                "{} Script ({}) not found in group ({})!",
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
                    println!("({count}) scripts found...");
                }
            }
            Err(error) => error.handle(),
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

fn parser(home_dir: &str)-> Result<(), Exception>{
    fs::create_dir(home_dir);

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
                    'h' | 'f' | 'p' | 'c' | 'g' | 's' => {
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
        Some('I') => install(flags, args, home_dir), 
        Some('R') => remove(flags, args, home_dir),
        Some('S') => sync(flags, args, home_dir),
        Some('Q') => query(flags, args, home_dir),
        None => Err(Exception::MissingOperation),
        _ => Ok(()),
    }
}

fn main(){
    let home_dir = dirs::home_dir()
        .unwrap()
        .into_os_string()
        .into_string()
        .unwrap() + "/.config/alps/";

    if let Err(error) = parser(&home_dir){
        error.handle();
    }
}