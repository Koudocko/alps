use colored::Colorize;
use std::{
    fs,
    env,
    path::{Path, PathBuf}, 
    process::Command,
    collections::HashSet, 
}; 
use crate::util;

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
    if args.is_empty(){
        eprintln!(
            "{} Expected group! (use -h for help)",
            "[!!!]".red()
        );
        std::process::exit(1);
    }

    if !Path::new(&(home_dir.to_owned() + &args[0])).is_dir(){
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
pub fn missing_flag(flags: &HashSet<char>){
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
    *args = args.clone()
        .into_iter()
        .filter_map(|group|{
            let exists = Path::new(&(home_dir.to_owned() + &group)).is_dir();

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
    }).collect();
}
pub fn invalid_packages(home_dir: &str, args: &mut Vec<String>, mode: bool, group: &str){
    *args = args.clone()
        .into_iter()
        .filter_map(|package|{
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
    }).collect();
}
pub fn invalid_configs(home_dir: &str, args: &mut Vec<String>, mode: bool, group: &str){
    *args = args.clone()
        .into_iter()
        .filter_map(|config|{
            if mode{
                let true_path = fs::canonicalize(PathBuf::from(&config));
                
                match true_path{
                    Ok(true_path) =>{
                        let true_path = true_path.into_os_string()
                            .into_string()
                            .unwrap();

                        let contains = util::read_label("[CONFIGS]", group, home_dir)
                            .split_whitespace()
                            .any(|entry| true_path == entry);

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
    }).collect();
}
pub fn invalid_scripts(home_dir: &str, args: &mut Vec<String>, mode: bool, group: &str){
    *args = args.clone()
        .into_iter()
        .filter_map(|script|{
            if mode{
                let contains = util::read_label("[SCRIPTS]", group, home_dir)
                    .split_whitespace()
                    .any(|entry| script.split('/').last().unwrap() == entry);

                match fs::canonicalize(&script){
                    Ok(_) =>{
                        if contains{
                            eprintln!(
                                "{} Script ({}) already installed to group!",
                                "[!]".yellow(),
                                script.yellow()
                            );
                            None
                        }
                        else{
                            Some(script)
                        }                                   
                    }
                    Err(_) =>{
                        eprintln!(
                            "{} Path to Script ({}) does not exist!",
                            "[!]".yellow(),
                            script.yellow()
                        );

                        None
                    }
                }
            }            
            else{
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
    }).collect();
}