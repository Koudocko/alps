use colored::Colorize;
use std::{
    fs,
    path::Path, 
    io::prelude::*,
    process::Command,
    io::ErrorKind,
}; 

pub fn get_entries(text: &String)-> impl Iterator<Item = &str>{ 
    text.split(['\r', '\n'])
        .filter(|x|{
            if x.is_empty(){
                false
            }
            else{
                true
            }
        })
}


pub fn dup_count(config: &str, group: &str, home_dir: &str)-> usize{
    let mut highest = 0;

    for entry in get_entries(&read_label("[CONFIGS]", group, home_dir))
    {
        let (name, index) = entry.rsplit_once('/').unwrap_or((entry, entry)).1
            .rsplit_once('_').unwrap_or((entry, entry));
        if config == name{
            if let Ok(index) = index.parse::<usize>(){
                if index > highest{
                    highest = index; 
                }
            }
        }

    }
    highest
}

pub fn to_userdir(config: &mut String){
    let home = dirs::home_dir()
        .unwrap()
        .into_os_string()
        .into_string()
        .unwrap();

    if let Some(segments) = config.split_once("home_dir")
    {
        *config = home + &segments.1; 
    }
}

pub fn to_template(config: &mut String){
    if let Some(segments) = config.split_once(&dirs::home_dir()
        .unwrap()
        .into_os_string()
        .into_string()
        .unwrap())
    {
        *config = "home_dir".to_owned() + &segments.1;
    }
}

pub fn help_menu(){
    println!("{} alps <operation> [...]", "usage:".white());
    println!("{}", "operations:".white());
    println!("   alps {{-Q --query}} [flags] : query installed groups and their contents in your config");
    println!("   alps {{-S --sync}} [flags] : sync your system with a group and their contents");       
    println!("   alps {{-I --install}} [flags] : install a group and their contents in your config");
    println!("   alps {{-R --remove}} [flags] : remove a group and their contents in your config");
    println!("   alps {{-E --edit}} [flags] : edit a group and their contents in your config");
    println!("{} use {{-h --help}} on any operation for list of flags", "hint:".white())
}

pub fn edit_file(file_path: &str, editor: &str){
    match Command::new(&editor)
        .arg(file_path) 
        .status()
    {
            Ok(_) =>{
                let file_name = file_path.split("/configs/")
                    .last()
                    .unwrap();

                println!(
                    "{} Editing file ({})...",
                    "[%]".cyan(),
                    file_name.cyan()
                );
            }
            Err(_) =>{
                eprintln!(
                    "{} Invalid editor! Update EDITOR environment variable...",
                    "[!]".yellow()
                );
                std::process::exit(1);
            }
    }
}

pub fn find(args: Vec<String>, label: &str, home_dir: &str, group: &String, mutate: impl Fn(&str)-> &str){
    let text = read_label(label, group, home_dir);

    if !args.is_empty(){
        let mut status = 0;

        for arg in &args{
            let mut found = vec![];

            for package in get_entries(&text){
                if arg == mutate(package){
                    found.push(package);
                }
            }

            let label = &label[1..label.len()-1].to_lowercase();
            if !found.is_empty(){
                for found in found{
                    println!(
                        "{} Found {}/{}/{} ",
                        "[?]".blue(),
                        group.blue(),
                        label.blue(),
                        found.blue()
                    );
                }
            }
            else{
                eprintln!(
                    "{} {}/{}/{} not found!",
                    "[!]".yellow(),
                    group.yellow(),
                    label.yellow(),
                    arg.yellow()
                );
                status += 1;
            }
        }

        std::process::exit(status);
    }
    else{
        let mut count = 0;
        for package in get_entries(&text){
            print!("{}, ", mutate(package).blue());
            count += 1;
        }
        println!("({count}) entries found...");
    }
}

pub fn reformat_config(labels: &Vec<String>, group: &str, home_dir: &str){
    let mut config_text = String::new();

    for label in labels{
        config_text.push_str(&(label.to_owned() + "\n"));

        for entry in get_entries(&read_label(label, group, home_dir)){
            config_text.push_str(&(entry.to_owned() + "\n"));
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

pub fn copy_dir<S, D>(src: S, dst: D)
where
    S: AsRef<Path>,
    D: AsRef<Path>,
{
    let path = Path::new(src.as_ref());

    if path.is_dir(){
        for dir in fs::read_dir(src).unwrap(){
            let dir = dir.unwrap();
            copy_dir(dir.path(), dst.as_ref().join(dir.file_name()));
        }
    }
    else if path.is_file(){
        let parent = dst.as_ref().parent().unwrap();

        if let Err(error) = fs::create_dir_all(parent){
            if error.kind() == ErrorKind::PermissionDenied
                &&
                Command::new("sudo")
                    .args(["mkdir", "-p", parent.to_str().unwrap()])
                    .output()
                    .is_err()
            {
                eprintln!(
                    "{} Command ({}) failed to run!",
                    "[!!!]".red(),
                    "mkdir".red()
                );
                std::process::exit(1);
            }
        }

        if let Err(error) = fs::copy(&src, &dst){
            if error.kind() == ErrorKind::PermissionDenied
                && 
                Command::new("sudo")
                    .args(["cp", "-r", "-T", src.as_ref().as_os_str().to_str().unwrap(), dst.as_ref().as_os_str().to_str().unwrap()])
                    .output()
                    .is_err()
            {
                eprintln!(
                    "{} Command ({}) failed to run!",
                    "[!!!]".red(),
                    "cd".red()
                );
                std::process::exit(1);
            }
        }
    }
}

pub fn read_label(label: &str, group:  &str, home_dir: &str)-> String{
    let excludes = vec![
        String::from("[PACKAGES]"),
        String::from("[CONFIGS]"),
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

pub fn config_write(group: &str, label: &str, entry: &str, home_dir: &str, mode: bool){
    let excludes = vec![
        String::from("[PACKAGES]"),
        String::from("[CONFIGS]"),
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
                get_entries(&read_label(segment, group, home_dir)).filter_map(|line|{
                    if *line != *entry{
                        Some(line.to_owned() + "\n")
                    }
                    else{
                        None
                    }
                }).collect::<String>()
            };
        
        if segment == label && mode{
            segments.push(segment.to_owned() + "\n" + entry + &list);
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
