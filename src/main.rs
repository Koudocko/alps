mod sift;
mod util;

use std::{
    fs,
    env, 
    path::Path,
    process::Command,
    collections::HashSet, 
    io::{ErrorKind},
}; 
use colored::Colorize;

fn install(flags: HashSet<char>, mut args: Vec<String>, home_dir: &str){
    sift::missing_flag(&flags);

    if flags.contains(&'h'){
        println!("usage: {{-I}} [flag]");
        println!("flags:");
        println!("-g [group(s)] : install group to config");
        println!("-f [group] [config(s)] : install file to group");
        println!("-p [group] [package(s)] : install package to group");
        println!("-s [group] [script(s)] : install script to group");
    } 
    else if flags.contains(&'g'){
        sift::missing_args(home_dir, &mut args, 1);
        sift::invalid_groups(home_dir, &mut args, true);

        for arg in args{
            if fs::create_dir(home_dir.to_owned() + &arg).is_ok(){
                println!(
                    "{} Created group ({})...",
                    "[+]".green(),
                    arg.green()
                );
            }
        }
    }
    else if flags.contains(&'p'){ 
        let mut group = String::new();

        sift::missing_group(home_dir, &mut args, &mut group);
        sift::missing_args(home_dir, &mut args, 1);
        sift::invalid_packages(home_dir, &mut args, true, &mut group);

        for arg in args{
            util::config_write(&group, "[PACKAGES]", &arg, home_dir, true);

            println!(
                "{} Installed {}/{}/{}",
                "[+]".green(),
                group.green(),
                "PACKAGES".green(),
                arg.green()
            );
        }
    }
    else if flags.contains(&'f'){
        let mut group = String::new();

        sift::missing_group(home_dir, &mut args, &mut group);
        sift::missing_args(home_dir, &mut args, 1);
        sift::invalid_configs(home_dir, &mut args, true, &mut group);

        fs::create_dir(home_dir.to_owned() + &group + "/configs");

        for arg in &args{
            util::config_write(&group, "[CONFIGS]", arg, home_dir, true);

            let arg_name = arg.split('/').last().unwrap();
            util::copy_dir(
                arg, 
                home_dir.to_owned()
                    + &group
                    + "/configs/"
                    + arg_name
            );
            println!(
                "{} Installed {}/{}/{}",
                "[+]".green(),
                group.green(),
                "CONFIGS".green(),
                arg_name.green()
            );
        }
    }
    else if flags.contains(&'s'){
        let mut group = String::new();

        sift::missing_group(home_dir, &mut args, &mut group);
        sift::missing_args(home_dir, &mut args, 1);
        sift::invalid_scripts(home_dir, &mut args, true, &mut group);

        fs::create_dir(home_dir.to_owned() + &group + "/scripts");
        
        for arg in &args{
            util::config_write(&group, "[SCRIPTS]", arg, home_dir, true);

            util::copy_dir(
                arg, 
                home_dir.to_owned()
                    + &group
                    + "/scripts/"
                    + arg
            );

            println!(
                "{} Installed {}/{}/{}",
                "[+]".green(),
                arg.green(),
                group.green(),
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

fn remove(flags: HashSet<char>, mut args: Vec<String>, home_dir: &str){
    sift::missing_flag(&flags);

    if flags.contains(&'h'){
        println!("usage: {{-R}} [flag]");
        println!("flags:");
        println!("-g [group(s)] : remove specified groups including contents");
        println!("-f [group] [configs] : remove specified config(s) of group");       
        println!("-p [group] [package(s)] : remove specified package(s) of group");
        println!("-s [group] [script(s)] : remove specified script(s) of group");       
    }
    else if flags.contains(&'g'){
        sift::missing_args(home_dir, &mut args, 1);
        sift::invalid_groups(home_dir, &mut args, false);

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
        let mut group = String::new();

        sift::missing_group(home_dir, &mut args, &mut group);
        sift::missing_args(home_dir, &mut args, 1);
        sift::invalid_configs(home_dir, &mut args, false, &mut group);


        for arg in &args{
            util::config_write(&group, "[CONFIGS]", arg, home_dir, false);

            let config_name = arg.split('/').last().unwrap();
            let config_path = home_dir.to_owned() + &group + "/configs/" + config_name;
            if Path::new(&arg).is_dir(){
                fs::remove_dir_all(config_path);
            }
            else if Path::new(&arg).is_file(){
                fs::remove_file(config_path);
            }

            println!(
                "{} Removed {}/{}/{}...",
                "[-]".green(),
                group.green(),
                "[CONFIGS]".green(),
                config_name.green()
            );
        }
    }
    else if flags.contains(&'s'){
        let mut group = String::new();

        sift::missing_group(home_dir, &mut args, &mut group);
        sift::missing_args(home_dir, &mut args, 1);
        sift::invalid_scripts(home_dir, &mut args, false, &mut group);

         for arg in &args{
            util::config_write(&group, "[SCRIPTS]", &arg, home_dir, false);
            fs::remove_file(arg);

            println!(
                "{} Removed {}/{}/{}...",
                "[-]".green(),
                group.green(),
                "[SCRIPTS]".green(),
                arg.green()
            );
        } 
    }
    else if flags.contains(&'p'){
        let mut group = String::new();

        sift::missing_group(home_dir, &mut args, &mut group);
        sift::missing_args(home_dir, &mut args, 1);
        sift::invalid_packages(home_dir, &mut args, false, &mut group);

        for arg in &args{
            util::config_write(&group, "[PACKAGES]", arg, home_dir, false);

            println!(
                "{} Removed {}/{}/{}...",
                "[-]".green(),
                group.green(),
                "[PACKAGES]".green(),
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

fn sync(flags: HashSet<char>, mut args: Vec<String>, home_dir: &str){
    let mut group = String::new();

    sift::missing_flag(&flags);
    sift::missing_group(home_dir, &mut args, &mut group);

    if flags.contains(&'h'){
        println!("usage: {{-S}} [flag]");
        println!("flags:");
        println!("-g [group] : sync system with all group contents");
        println!("-f [group] : sync system with only group configs");       
        println!("-p [group] : sync system with only group packages");
        println!("-s [group] : sync system with only group scripts");              
    }
    else if flags.contains(&'p'){
        let packages = util::read_label("[PACKAGES]", &group, home_dir)
            .split_whitespace()
            .filter_map(|package|{
                if Command::new("pacman")
                    .args(["-Q", package])
                    .output()
                    .unwrap()
                    .status
                    .success()
                {
                    eprintln!(
                        "{} Package ({}) already installed to system!",
                        "[!]".yellow(),
                        package.yellow()
                    );
                    None
                }
                else{
                    if Command::new("pacman")
                        .args(["-Ss", &("^".to_owned() + &package + "$")])
                        .output()
                        .unwrap()
                        .status
                        .success()
                    {
                        println!(
                            "{} Installing package ({}) to system...",
                            "[+]".purple(),
                            package.purple()
                        );
                        Some(package.to_owned())
                    }
                    else{
                        eprintln!(
                            "{} Package ({}) does not exist in repository!",
                            "[!]".yellow(),
                            package.yellow()
                        );
                        None
                    }
                }
            }
            ).collect::<Vec<String>>();
        
        if !packages.is_empty(){
            match Command::new("sudo")
                .args(["pacman", "-S"])
                .args(packages.as_slice())
                .status()
            {
                Ok(_) =>{
                    println!(
                        "{} Successfully synced packages...",
                        "[~]".purple()
                    )
                }
                Err(_) =>{
                    eprintln!(
                        "{} Failed to sync packages!",
                        "[!!!]".red()
                    );

                    std::process::exit(1);
                }
            }
        }
        else{
            eprintln!(
                "{} No packages to sync in group ({})!",
                "[!]".yellow(),
                group.yellow()
            );
        }
    }
    else if flags.contains(&'f'){
        let text = util::read_label("[CONFIGS]", &group, home_dir);

        if !text.is_empty(){
            for path in text.split_whitespace()
            {
                let path_name = path.split('/').last().unwrap();
                util::copy_dir(home_dir.to_owned() + &group + "/configs/" + path_name, path);
    
                println!(
                    "{} Synced config ({}) successfully!",
                    "[~]".purple(),
                    path_name.purple(),
                )
            }
        }
        else{
            eprintln!(
                "{} No configs to sync in group ({})",
                "[!]".yellow(),
                group.yellow()
            );
        }
    }
    else if flags.contains(&'s'){
        let text = util::read_label("[SCRIPTS]", &group, home_dir);

        if !text.is_empty(){
            for script in text.split_whitespace(){
                let mut handle = Command::new("/".to_owned() + home_dir + &group + "/scripts/" + script);

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
        else{
            eprintln!(
                "{} No scripts to sync in group ({})",
                "[!]".yellow(),
                group.yellow()
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

fn query(flags: HashSet<char>, mut args: Vec<String>, home_dir: &str){
    sift::missing_flag(&flags);

    if flags.contains(&'h'){
        println!("usage: {{-Q}} [flag]");
        println!("flags:");
        println!("-g [?group(s)] : query installed group(s)");
        println!("-f [group] [?config(s)]: query installed configs of a group");       
        println!("-p [group] [?packages(s)] : query installed packages of a group");
        println!("-s [group] [?scripts(s)] : query installed scripts of a group");      
        println!("NOTE: ? = optional");
    }
    else if flags.contains(&'g'){
        //Credit Raforawesome (aka God)
        /*let groups: Vec<fs::DirEntry> = fs::read_dir(home_dir).unwrap()
            .filter(|entry|{
                let entry = entry.as_ref().unwrap()
                    .file_type().unwrap();

                entry.is_dir()
            }).map(|x| x.unwrap()).collect();

        if !groups.is_empty(){
            if !args.is_empty(){
                let mut status = 0;
    
                for arg in &args{
                    let contains = groups.into_iter().any(|group|{
                        let group = &group
                            .file_name()
                            .into_string()
                            .unwrap();
    
                        arg == group
                    });
                    
                    if contains{
                        println!(
                            "{} Group ({}) found...",
                            "[?]".blue(),
                            arg.blue()
                        );
                    }
                    else{
                        eprintln!(
                            "{} Group ({}) not found!",
                            "[!]".yellow(),
                            arg.yellow()
                        );
                        status += 1;
                    }               
                }
    
                std::process::exit(status);
            }
            else{
                for group in &groups{
                    let group = group.file_name().into_string().unwrap();
    
                    println!(
                        "{} :: ({}) packages :: ({}) configs :: ({}) scripts", 
                        group.blue(),
                        read_label("[PACKAGES]", &group, home_dir).split_whitespace().count(),
                        read_label("[CONFIGS]", &group, home_dir).split_whitespace().count(),
                        read_label("[SCRIPTS]", &group, home_dir).split_whitespace().count()
                    );
                }
                println!("({}) groups found...", groups.len());
            }
        }
        else{
            eprintln!(
                "{} No groups installed!",
                "[!]".yellow()
            );
        }*/
    }
    else if flags.contains(&'p'){
        let group = String::new();
        util::find(args, "[PACKAGES]", home_dir, &group);
    }
    else if flags.contains(&'f'){
        let mut group = String::new();
        sift::missing_group(home_dir, &mut args, &mut group);
        util::find(args, "[CONFIGS]", home_dir, &group);
    }
    else if flags.contains(&'s'){
        let mut group = String::new();
        sift::missing_group(home_dir, &mut args, &mut group);
        util::find(args, "[SCRIPTS]", home_dir, &group);
    }
    else{
        eprintln!(
            "{} Invalid flag! (use -h for help)",
            "[!!!]".red()
        );
        
        std::process::exit(1);
    }
}

fn parser(home_dir: &str){
    fs::create_dir(home_dir);

    let p_args: Vec<_> = env::args().collect();
    if p_args.len() == 1{
        eprintln!(
            "{} Expected arguments! (use -h for help)",
            "[!!!]".red()
        );

        std::process::exit(1);
    }

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
                            eprintln!(
                                "{} Cannot use more than one operation! (use -h for help)",
                                "[!!!]".red()
                            );

                            std::process::exit(1);
                        }
                    }
                    'h' | 'f' | 'p' | 'c' | 'g' | 's' => {
                        flags.insert(flag);
                    }
                    op =>{ 
                        eprintln!(
                            "{} Invalid operation ({})! (use -h for help)",
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

    match mode{
        Some('I') => install(flags, args, home_dir), 
        Some('R') => remove(flags, args, home_dir),
        Some('S') => sync(flags, args, home_dir),
        Some('Q') => query(flags, args, home_dir),
        None => eprintln!(
            "{} Expected operation! (use -h for help)",
            "[!!!]".red()
        ),
        _ => (),
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