use std::{
    fs,
    env, 
    path::{Path, PathBuf}, 
    process::Command,
    collections::HashSet, 
    io::{prelude::*, ErrorKind}, 
}; 
use colored::Colorize;

#[derive(Debug)]
enum CW{
    MissingGroup(Vec<String>),
    MissingFlag(HashSet<char>),
    MissingArgs(Vec<String>),
    InvalidGroups(Vec<String>, bool),
    InvalidPackages(Vec<String>, bool),
    InvalidConfigs(Vec<String>, bool),
    InvalidScripts(Vec<String>, bool),
}

fn bounds_check(exceptions: &[CW]){
    let home_dir = dirs::home_dir()
        .unwrap()
        .into_os_string()
        .into_string()
        .unwrap() + "/.config/alps/";

    for exception in exceptions{
        match exception{
            CW::MissingGroup(args) =>{
                if args.is_empty(){
                    eprintln!(
                        "{} Expected group! (use -h for help)",
                        "[!!!]".red()
                    );
                    std::process::exit(1);
                }

                if !Path::new(&(home_dir.to_owned() + &args[0])).is_dir(){
                    eprintln!(
                        "{} Invalid group! (use -h for help)",
                        "[!!!]".red(),
                    );

                    std::process::exit(1);
                }
            }
            CW::MissingFlag(flags) =>{
                if flags.is_empty(){
                    eprintln!(
                        "{} Expected flag! (use -h for help)",
                        "[!!!]".red()
                    );
                }
            }
            CW::MissingArgs(args) =>{
                if args.is_empty(){
                    eprintln!(
                        "{} Expected arguments! (use -h for help)",
                        "[!!!]".red()
                    );
                    std::process::exit(1);
                }
            }
            CW::InvalidGroups(args, mode) =>{
                let mut groups = Vec::<String>::new();

                for arg in args{
                    if *mode{
                        if Path::new(&(home_dir.to_owned() + arg)).is_dir(){
                            eprintln!(
                                "{} Group ({}) already installed!",
                                "[!]".yellow(),
                                arg.yellow()
                            );

                            continue;
                        }
                    }
                    else{
                        if !Path::new(&(home_dir.to_owned() + arg)).is_dir(){
                            eprintln!(
                                "{} Group ({}) not installed!",
                                "[!]".yellow(),
                                arg.yellow()
                            );
    
                            continue;
                        }
                    }

                    groups.push(arg.to_string());
                }

                args = &groups;
            }
            CW::InvalidPackages(args, mode) =>{
                let mut packages = Vec::<String>::new();
               
                for arg in &args[1..]{
                    let contains = read_label("[PACKAGES]", &args[0], &home_dir)
                        .split_whitespace()
                        .any(|ele| *arg == ele);

                    if *mode{
                        if contains{
                            eprintln!(
                                "{} Package ({}) already installed!",
                                "[!]".yellow(),
                                arg.yellow()
                            );

                            continue;
                        }
                    
                        let mut handle = Command::new("pacman")
                            .args(["-Ss", &("^".to_owned() + arg + "$")])
                            .output()
                            .unwrap();

                        if !handle.status.success(){
                            eprintln!(
                                "{} Package ({}) does not exist!",
                                "[!]".yellow(),
                                arg.yellow()
                            );

                            continue;
                        }
                    }

                    if !contains{
                        eprintln!(
                            "{} Package ({}) not installed!",
                            "[!]".yellow(),
                            arg.yellow()
                        );

                        continue;
                    }

                    packages.push(arg.to_owned());
                }

                args = &packages;
            }
            CW::InvalidConfigs(args, mode) =>{
                let mut configs = Vec::<String>::new();

                for arg in &args[1..]{
                    let mut config = String::new();

                    if *mode{
                        match fs::canonicalize(PathBuf::from(arg)){
                            Ok(true_path) =>{
                                config = true_path
                                    .into_os_string()
                                    .into_string()
                                    .unwrap();

                                let contains = read_label("[CONFIGS]", &args[0], &home_dir)
                                    .split_whitespace()
                                    .any(|ele| config == ele);
        
                                if contains{
                                    eprintln!(
                                        "{} Config ({}) already installed!",
                                        "[!]".yellow(),
                                        arg.yellow()
                                    );
        
                                    continue;
                                }
                            }
                            Err(_) =>{
                                eprintln!(
                                    "{} Config ({}) does not exist!",
                                    "[!]".yellow(),
                                    arg.yellow()
                                );
    
                                continue;
                            }
                        }
                   }
                   else{
                        let contains = read_label("[CONFIGS]", &args[0], &home_dir)
                            .split_whitespace()
                            .any(|ele|{
                                config = ele.to_string();
                                arg == ele.split('\n').last().unwrap()
                            });

                        if !contains{
                            eprintln!(
                                "{} Config ({}) not installed!",
                                "[!]".yellow(),
                                arg.yellow()
                            );

                            continue;
                        }
                    }

                    configs.push(config);
                }

                args = &configs;
            }
            CW::InvalidScripts(args, mode) =>{
                let mut scripts = Vec::<String>::new();

                for arg in args{
                    if *mode{
                        match fs::canonicalize(PathBuf::from(arg)){
                            Ok(_) =>{
                                let contains = read_label("[SCRIPTS]", &args[0], &home_dir)
                                    .split_whitespace()
                                    .any(|ele| arg == ele);
        
                                if contains{
                                    eprintln!(
                                        "{} Script ({}) already installed!",
                                        "[!]".yellow(),
                                        arg.yellow()
                                    );
        
                                    continue;
                                }
                            }
                            Err(_) =>{
                                eprintln!(
                                    "{} Script ({}) does not exist!",
                                    "[!]".yellow(),
                                    arg.yellow()
                                );
    
                                continue;
                            }
                        }
                    }
                    else{
                        let contains = read_label("[SCRIPTS]", &args[0], &home_dir)
                            .split_whitespace()
                            .any(|ele| arg == ele);
    
                        if !contains{
                            eprintln!(
                                "{} Script ({}) not installed!",
                                "[!]".yellow(),
                                arg.yellow()
                            );
    
                            continue;
                        }
                    }
    
                    scripts.push(arg.to_string());
                }
    
                args = &scripts;
            }
            _ => (),
        }
    }
}

fn reformat_config(labels: &Vec<String>, group: &str, home_dir: &str){
    let mut config_text = String::new();

    for label in labels{
        config_text.push_str(&(label.to_owned() + "\n"));

        for entry in read_label(label, group, home_dir).split_whitespace(){
            config_text.push_str (&(entry.to_owned() + "\n"));
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

fn read_label(label: &str, group:  &str, home_dir: &str)-> String{
    let excludes = vec![
        String::from("[PACKAGES]"),
        String::from("[PATHS]"),
        String::from("[SCRIPTS]")
    ];

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

        text.to_string()
    }
    else{
        String::new()
    }
}

fn config_write(group: &str, label: &str, entry: &str, home_dir: &str, mode: bool){
    let excludes = vec![
        String::from("[PACKAGES]"),
        String::from("[PATHS]"),
        String::from("[SCRIPTS]")
    ];

    reformat_config(&excludes, group, home_dir);

    let mut segments: Vec<String> = Vec::new();
    for segment in &excludes{
        let list = 
            if mode{
                read_label(segment, group, home_dir)
            }
            else{
                read_label(segment, group, home_dir).split(entry).collect::<String>()
            };
        
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
        .open(home_dir.to_owned() + group + "/" + group + ".conf")
        .unwrap();           

    for mut segment in segments{
        if !segment.ends_with('\n'){
            segment.push('\n');
        }

        handle.write_all(segment.as_bytes()).unwrap();
    }

    reformat_config(&excludes, group, home_dir);
}

fn install(flags: HashSet<char>, args: Vec<String>, home_dir: &str){
    bounds_check(&[CW::MissingFlag(flags)]);

    if flags.contains(&'h'){
        println!("usage: {{-I}} [flag]");
        println!("flags:");
        println!("-g [group(s)] : install group to config");
        println!("-f [group] [config(s)] : install file to group");
        println!("-p [group] [package(s)] : install package to group");
        println!("-s [group] [script(s)] : install script to group");
    } 
    else if flags.contains(&'g'){
        bounds_check(
            &[CW::MissingArgs(args), 
            CW::InvalidGroups(args, true)]
        );

        for arg in args{
            if let Ok(_) = fs::create_dir(home_dir.to_owned() + &arg){
                println!(
                    "{} Created group ({})...",
                    "[+]".green(),
                    arg.green()
                );
            }
        }
    }
    else if flags.contains(&'p'){ 
        bounds_check(
            &[CW::MissingGroup(args), 
            CW::MissingArgs(args), 
            CW::InvalidPackages(args, true)]
        );

        for arg in args{
            config_write(&args[0], "[PACKAGES]", &arg, home_dir, true);

            println!(
                "{} Installed {}/{}/{}",
                "[+]".green(),
                arg.green(),
                &args[0].green(),
                "PACKAGES".green()
            );
        }
    }
    else if flags.contains(&'f'){
        bounds_check(
            &[CW::MissingGroup(args), 
            CW::MissingArgs(args), 
            CW::InvalidConfigs(args, true)]
        );

        fs::create_dir(home_dir.to_owned() + &args[0] + "/configs");

        for arg in args{
            config_write(&args[0], "[CONFIGS]", &arg, home_dir, true);

            copy_dir(
                arg, 
                (home_dir.to_owned()
                    + &args[0]
                    + "/configs/"
                    + (arg
                        .split('/')
                        .last()
                        .unwrap()
                    )
                ) 
            );

            println!(
                "{} Installed {}/{}/{}",
                "[+]".green(),
                arg.green(),
                &args[0].green(),
                "CONFIGS".green()
            );
        }
    }
    else if flags.contains(&'s'){
        bounds_check(
            &[CW::MissingGroup(args), 
            CW::MissingArgs(args), 
            CW::InvalidScripts(args, true)]
        );

        fs::create_dir(home_dir.to_owned() + &args[0] + "/scripts");
        
        for arg in args{
            config_write(&args[0], "[SCRIPTS]", &arg, home_dir, true);

            copy_dir(
                arg, 
                (home_dir.to_owned()
                    + &args[0]
                    + "/scripts/"
                    + &arg
                )   
            );

            println!(
                "{} Installed {}/{}/{}",
                "[+]".green(),
                arg.green(),
                &args[0].green(),
                "SCRIPTS".green()
            );

        }
    }
    else{
        eprintln!(
            "{} Invalid flag! (use -h for help)",
            "[!!!]".red()
        );
        
        std::process::exit(1);
    }
}

fn remove(flags: HashSet<char>, args: Vec<String>, home_dir: &str){
    bounds_check(&[CW::MissingFlag(flags)]);

    if flags.contains(&'h'){
        println!("usage: {{-R}} [flag]");
        println!("flags:");
        println!("-g [group(s)] : remove specified groups including contents");
        println!("-f [group] [configs] : remove specified config(s) of group");       
        println!("-p [group] [package(s)] : remove specified package(s) of group");
        println!("-s [group] [script(s)] : remove specified script(s) of group");       
    }
    else if flags.contains(&'g'){
        bounds_check(
            &[CW::MissingArgs(args), 
            CW::InvalidGroups(args, false)]
        );

        for arg in args{
            if let Ok(_) = fs::remove_dir_all(home_dir.to_owned() + &arg){
                println!(
                    "{} Removed group ({})...",
                    "[-]".green(),
                    arg.green()
                );
            }
        }
    }
    else if flags.contains(&'f'){
        bounds_check(
            &[CW::MissingGroup(args),
            CW::MissingArgs(args),
            CW::InvalidConfigs(args, false)]
        );

        for arg in args{
            config_write(&args[0], "[CONFIGS]", &arg, home_dir, false);

            if Path::new(&arg).is_dir(){
                fs::remove_dir_all(arg);
            }
            else if Path::new(&arg).is_file(){
                fs::remove_file(arg);
            }

            println!(
                "{} Removed {}/{}/{}...",
                "[-]".green(),
                &args[0].green(),
                "configs".green(),
                arg.green()
            );
        }
    }
    else if flags.contains(&'s'){
        bounds_check(
            &[CW::MissingGroup(args),
            CW::MissingArgs(args),
            CW::InvalidScripts(args, false)]
        );

         for arg in args{
            config_write(&args[0], "[SCRIPTS]", &arg, home_dir, false);
            fs::remove_file(arg);

            println!(
                "{} Removed {}/{}/{}...",
                "[-]".green(),
                &args[0].green(),
                "scripts".green(),
                arg.green()
            );
        } 
    }
    else if flags.contains(&'p'){
        bounds_check(
            &[CW::MissingGroup(args),
            CW::MissingArgs(args),
            CW::InvalidPackages(args, false)]
        );

        for arg in args{
            config_write(&args[0], "[PACKAGES]", &arg, home_dir, false);

            println!(
                "{} Removed {}/{}/{}...",
                "[-]".green(),
                &args[0].green(),
                "packages".green(),
                arg.green()
            );
        }
    }
    else{
        eprintln!(
            "{} Invalid flag! (use -h for help)",
            "[!!!]".red()
        );
        
        std::process::exit(1);
    }
}

fn sync(flags: HashSet<char>, args: Vec<String>, home_dir: &str){
    if flags.contains(&'h'){
        println!("usage: {{-S}} [flag]");
        println!("flags:");
        println!("-g [group] : sync system with all group contents");
        println!("-f [group] : sync system with only group configs");       
        println!("-p [group] : sync system with only group packages");
        println!("-s [group] : sync system with only group scripts");              
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
        eprintln!(
            "{} Invalid flag! (use -h for help)",
            "[!!!]".red()
        );
        
        std::process::exit(1);
    }
}

fn query(flags: HashSet<char>, args: Vec<String>, home_dir: &str)-> Result<(), CW>{
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

            return Err(CW::Status(status));
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

                    return Err(CW::Status(status));
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

                    return Err(CW::Status(status));
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

                    return Err(CW::Status(status));
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
            return Err(CW::MissingOption);
        }

        return Err(CW::InvalidOption);
    }

    Ok(())
}

fn parser(home_dir: &str){
    fs::create_dir(home_dir);

    let p_args: Vec<_> = env::args().collect();
    if p_args.len() == 1{ return Err(CW::PlainRun); }

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
                            return Err(CW::DuplicateOperation);
                        }
                    }
                    'h' | 'f' | 'p' | 'c' | 'g' | 's' => {
                        flags.insert(flag);
                    }
                    op =>{ 
                        return Err(CW::InvalidOperation(op));
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
        None => Err(CW::MissingOperation),
        _ => Ok(()),
    }
}

fn main(){
    let home_dir = dirs::home_dir()
        .unwrap()
        .into_os_string()
        .into_string()
        .unwrap() + "/.config/alps/";

    parser(&home_dir);
}
