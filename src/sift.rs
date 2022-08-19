use colored::Colorize;
use std::{
    fs,
    env,
    path::{Path, PathBuf}, 
    process::Command,
    collections::HashSet, 
}; 
use crate::util;

pub fn invalid_operation(operation: &str){
    let hyphen = 
        if operation.len() == 1{ "-" }
        else { "" };

    eprintln!(
        "{} Invalid operation ({}{})!",
        "[!!!]".red(),
        hyphen.red(),
        operation.red()
    );
    util::help_menu();

    std::process::exit(1);
}

pub fn duplicate_operation(mode: &mut Option<String>, flag: String){
    if *mode == None{
       *mode = Some(flag); 
    }
    else{
        eprintln!(
            "{} Cannot use more than one operation!",
            "[!!!]".red()
        );

        std::process::exit(1);
    }
}

pub fn invalid_flag(){
    eprintln!(
        "{} Invalid flag! (use -h for help)",
        "[!!!]".red()
    );
    
    std::process::exit(1);
}

pub fn missing_editor(editor: &mut String){
    match env::var("EDITOR"){
        Ok(extract) => *editor = extract,
        Err(_) =>{
            eprintln!(
                "{} Editor not found! Set environment variable EDITOR to continue...",
                "[!!!]".red()
            );
            std::process::exit(1);
        } 
    }
}

pub fn missing_group(home_dir: &str, args: &mut Vec<String>, group: &mut String){
    let excludes = vec![
        String::from(".git"), 
        String::from(".."), 
        String::from(".")
    ];

    if args.is_empty(){
        eprintln!(
            "{} Expected group! (use -h for help)",
            "[!!!]".red()
        );
        std::process::exit(1);
    }

    if !Path::new(&(home_dir.to_owned() + &args[0])).is_dir() || excludes.contains(&args[0]){
        eprintln!(
            "{} Invalid group ({})! (use -h for help)",
            "[!!!]".red(),
            &args[0].red()
        );
        std::process::exit(1);
    }
    *group = args[0].clone();
    args.remove(0);
}

pub fn missing_flag(flags: &HashSet<String>){
    if flags.is_empty(){
        eprintln!(
            "{} Expected flag! (use -h for help)",
            "[!!!]".red()
        );
        std::process::exit(1);
    }
}

pub fn missing_args(args: &mut Vec<String>, len: usize){
    if args.len() < len{
        eprintln!(
            "{} Expected arguments! (use -h for help)",
            "[!!!]".red()
        );
        std::process::exit(1);
    }
}

pub fn invalid_groups(home_dir: &str, args: &mut Vec<String>, mode: bool){
    let excludes = vec![
        String::from(".git"), 
        String::from(".."), 
        String::from(".")
    ];

    *args = args.clone()
        .into_iter()
        .filter_map(|group|{
            let exists = Path::new(&(home_dir.to_owned() + &group)).is_dir();

            if excludes.contains(&group){
                eprintln!(
                    "{} Invalid group ({})! (use -h for help)",
                    "[!]".yellow(),
                    &args[0].yellow()
                );
                None
            }
            else{
                if mode{
                    if exists{
                        eprintln!(
                            "{} Group ({}) already installed!",
                            "[!]".yellow(),
                            group.yellow()
                        );
                        None
                    }
                    else{
                        Some(group)
                    }
                }
                else{
                    if !exists{
                        eprintln!(
                            "{} Group ({}) does not exist!",
                            "[!]".yellow(),
                            group.yellow()
                        );
                        None
                    }
                    else{
                        Some(group)
                    }
                }
            }
    }).collect();
}

pub fn invalid_packages(home_dir: &str, args: &mut Vec<String>, mode: bool, group: &str){
    println!(
        "{} Processing arguments. Please wait...",
        "[*]".magenta()
    );

    let mut passed = HashSet::new();

    *args = args.clone()
        .into_iter()
        .filter_map(|package|{
            if passed.contains(&package){
                eprintln!(
                    "{} Removing duplicate argument ({})!",
                    "[!]".yellow(),
                    package.yellow()
                );
                None
            }
            else{
                passed.insert(package.to_owned());

                let exists = util::read_label("[PACKAGES]", group, home_dir)
                    .split_whitespace()
                    .any(|entry| package == entry);
    
                if mode{
                    if exists{
                        eprintln!(
                            "{} Package ({}) already installed to group!",
                            "[!]".yellow(),
                            package.yellow()
                        );
                        None
                    }
                    else{
                        let handle = Command::new("pacman")
                            .args(["-Ss", &("^".to_owned() + &package + "$")])
                            .output()
                            .unwrap();
    
                        if !handle.status.success(){ 
                            eprintln!(
                                "{} Package ({}) does not exist in repository!",
                                "[!]".yellow(),
                                package.yellow()
                            );
                            None
                        }
                        else{
                            Some(package)
                        }
                    }
                }
                else{
                    if !exists{
                        eprintln!(
                            "{} Package ({}) does not exist in group!",
                            "[!]".yellow(),
                            package.yellow()
                        );
                        None
                    }
                    else{
                        Some(package)
                    }
                }
            }
    }).collect();
}

pub fn invalid_configs(home_dir: &str, args: &mut Vec<String>, mode: bool, group: &str){
    println!(
        "{} Processing arguments. Please wait...",
        "[*]".magenta()
    );

    let mut passed = HashSet::new();

    *args = args.clone()
        .into_iter()
        .filter_map(|config|{
            if mode{
                match fs::canonicalize(PathBuf::from(&config)){
                    Ok(true_path) =>{
                        let true_path = true_path.into_os_string()
                            .into_string()
                            .unwrap();

                        if passed.contains(&true_path){
                            eprintln!(
                                "{} Removing duplicate argument ({})!",
                                "[!]".yellow(),
                                config.yellow()
                            );
                            None
                        }
                        else{
                            passed.insert(true_path.to_owned());

                            let mut generic_path = true_path.to_owned();
                            util::to_template(&mut generic_path);

                            let contains = util::read_label("[CONFIGS]", group, home_dir)
                                .split_whitespace()
                                .any(|entry| generic_path == entry.rsplit_once('_').unwrap_or((entry, entry)).0);
    
                            if contains{
                                eprintln!(
                                    "{} Config ({}) already installed to group!",
                                    "[!]".yellow(),
                                    config.yellow()
                                );
                                None
                            }
                            else{
                                Some(true_path)
                            }
                        }
                    }
                    Err(_) =>{
                        eprintln!(
                            "{} Path to config ({}) does not exist!",
                            "[!]".yellow(),
                            config.yellow()
                        );

                        None
                    }
                }
            }
            else{
                let mut config_path = String::new();
                let contains = util::read_label("[CONFIGS]", group, home_dir)
                    .split_whitespace()
                    .any(|entry|{
                        config_path = entry.to_string();
                        config == entry.split('/').last().unwrap()
                    });

                if passed.contains(&config_path){
                    eprintln!(
                        "{} Removing duplicate argument ({})!",
                        "[!]".yellow(),
                        config.yellow()
                    );
                    None   
                }
                else{
                    passed.insert(config_path.to_owned());

                    if !contains{
                        eprintln!(
                            "{} Config ({}) does not exist in group!",
                            "[!]".yellow(),
                            config.yellow()
                        );                               
                        None
                    }
                    else{
                        Some(config_path)
                    }
                }
            }
    }).collect();
}

pub fn invalid_scripts(home_dir: &str, args: &mut Vec<String>, mode: bool, group: &str){
    println!(
        "{} Processing arguments. Please wait...",
        "[*]".magenta()
    );

    let mut passed = HashSet::new();

    *args = args.clone()
        .into_iter()
        .filter_map(|script|{
            if mode{
                let mut script_name = String::new();

                let contains = util::read_label("[SCRIPTS]", group, home_dir)
                    .split_whitespace()
                    .any(|entry|{
                        script_name = script.split('/').last().unwrap().to_string();
                        script_name == entry
                    });
                
                if passed.contains(&script_name){
                    eprintln!(
                        "{} Removing duplicate argument ({})",
                        "[!]".yellow(),
                        script_name.yellow()
                    );
                    None
                }
                else{
                    passed.insert(script_name.to_owned());

                    match fs::canonicalize(&script){
                        Ok(_) =>{
                            if contains{
                                eprintln!(
                                    "{} Script ({}) already installed to group!",
                                    "[!]".yellow(),
                                    script_name.yellow()
                                );
                                None
                            }
                            else{
                                if !Path::new(&script).is_file(){
                                    eprintln!(
                                        "{} Script ({}) is not a file!",
                                        "[!]".yellow(),
                                        script.yellow()
                                    );
    
                                    None
                                }
                                else{
                                    Some(script)
                                }
                            }                                   
                        }
                        Err(_) =>{
                            eprintln!(
                                "{} Path to script ({}) does not exist!",
                                "[!]".yellow(),
                                script.yellow()
                            );
                            None
                        }
                    }
                }
            }            
            else{
                if passed.contains(&script){
                    eprintln!(
                        "{} Removing duplicate argument ({})",
                        "[!]".yellow(),
                        script.yellow()
                    );
                    None
                }
                else{
                    passed.insert(script.to_owned());

                    let contains = util::read_label("[SCRIPTS]", group, home_dir)
                        .split_whitespace()
                        .any(|entry| script == entry );
    
                    if !contains{
                        eprintln!(
                            "{} Script ({}) does not exist in group!",
                            "[!]".yellow(),
                            script.yellow()
                        );
                        None
                    }
                    else{
                        Some(script)
                    }
                }
            }
    }).collect();
}
