mod sift;
mod util;
mod flag;

use std::{
    fs,
    env, 
    collections::HashSet, 
}; 
use colored::Colorize;

fn install(flags: HashSet<String>, args: Vec<String>, home_dir: &str){
    sift::missing_flag(&flags);

    if flags.contains("h") || flags.contains("--help"){
        flag::install_help();
    } 
    else if flags.contains("g") || flags.contains("--group"){
        flag::install_group(args, home_dir);
    }
    else if flags.contains("p") || flags.contains("--package"){ 
        flag::install_package(args, home_dir);
    }
    else if flags.contains("c") || flags.contains("--config"){
        flag::install_config(args, home_dir);
    }
    else if flags.contains("s") || flags.contains("--script"){
        flag::install_script(args, home_dir);
    }
    else{
        sift::invalid_flag();
    }
}

fn remove(flags: HashSet<String>, args: Vec<String>, home_dir: &str){
    sift::missing_flag(&flags);

    if flags.contains("h") || flags.contains("--help"){
        flag::remove_help();
    }
    else if flags.contains("g") || flags.contains("--group"){
        flag::remove_group(args, home_dir);
    }
    else if flags.contains("c") || flags.contains("--config"){
        flag::remove_config(args, home_dir);
    }
    else if flags.contains("s") || flags.contains("--script"){
        flag::remove_script(args, home_dir);
    }
    else if flags.contains("p") || flags.contains("--package"){
        flag::remove_package(args, home_dir);
    }
    else{
        sift::invalid_flag();
    }
}

fn sync(flags: HashSet<String>, mut args: Vec<String>, home_dir: &str){
    sift::missing_flag(&flags);

    if flags.contains("h") || flags.contains("--help"){
        flag::sync_help();
    }
    else if flags.contains("g") || flags.contains("--group"){
        let mut group = String::new();
        sift::missing_group(home_dir, &mut args, &mut group);

        flag::sync_group(home_dir, &group);
    }
    else if flags.contains("p") || flags.contains("--package"){
        let mut group = String::new();
        sift::missing_group(home_dir, &mut args, &mut group);

        flag::sync_package(home_dir, &group);
    }
    else if flags.contains("c") || flags.contains("--config"){
        let mut group = String::new();
        sift::missing_group(home_dir, &mut args, &mut group);

        flag::sync_config(home_dir, &group);
    }
    else if flags.contains("s") || flags.contains("--script"){
        let mut group = String::new();
        sift::missing_group(home_dir, &mut args, &mut group);

        flag::sync_script(home_dir, &group);
    }
    else{
        sift::invalid_flag();
    }
}

fn query(flags: HashSet<String>, args: Vec<String>, home_dir: &str){
    sift::missing_flag(&flags);

    if flags.contains("h") || flags.contains("--help"){
        flag::query_help();
    }
    else if flags.contains("g") || flags.contains("--group"){
        flag::query_group(args, home_dir);
    }
    else if flags.contains("p") || flags.contains("--package"){
        flag::query_package(args, home_dir);
    }
    else if flags.contains("c") || flags.contains("--config"){
        flag::query_config(args, home_dir);
    }
    else if flags.contains("s") || flags.contains("--script"){
        flag::query_script(args, home_dir);
    }
    else{
        sift::invalid_flag();
    }
}

fn edit(flags: HashSet<String>, args: Vec<String>, home_dir: &str){
    let mut editor = String::new();
    sift::missing_flag(&flags);
    sift::missing_editor(&mut editor);

    if flags.contains("h") || flags.contains("--help"){
        flag::edit_help();
    }   
    else if flags.contains("g") || flags.contains("--group"){
        flag::edit_group(args, home_dir, editor);
    }
    else if flags.contains("c") || flags.contains("--config"){
        flag::edit_config(args, home_dir, editor);
    }
    else if flags.contains("s") || flags.contains("--script"){
        flag::edit_script(args, home_dir, editor);
    }
    else{
        sift::invalid_flag();
    }
}

fn parser(home_dir: &str){
    let p_args: Vec<_> = env::args().collect();

    let mut args: Vec<String> = Vec::new();
    let mut flags = HashSet::new();
    let mut passed = HashSet::new();
    let mut mode = None; 

    if p_args.len() > 1{
        for arg in &p_args[1..]{
            if arg.len() > 1 && &arg[..2] == "--"{
                if arg.len() > 2{
                    match arg.as_str(){
                        "--install" | "--remove" | "--sync" | "--query" | "--edit" =>{
                            sift::duplicate_operation(&mut mode, arg.to_owned());
                        }
                        "--help" | "--group" | "--package" | "--config" | "--script" =>{
                            flags.insert(arg.to_owned());
                        }
                        arg => sift::invalid_operation(arg),
                    }
                }
            }
            else if &arg[..1] == "-"{
                for flag in arg[1..].chars(){
                    let flag = flag.to_string();
    
                    match flag.as_str(){
                        "I" | "R" | "S" | "Q" | "E" =>{
                            sift::duplicate_operation(&mut mode, flag);
                        }
                        "h" | "p" | "c" | "g" | "s" =>{
                            flags.insert(flag);
                        }
                        flag => sift::invalid_operation(flag),
                    }
                }           
            }
            else{
                if passed.contains(arg){
                    eprintln!(
                        "{} Removing duplicate argument ({})",
                        "[!]".yellow(),
                        arg.yellow()
                    );
                }
                else{
                    passed.insert(arg);
                    args.push(arg.to_owned());
                }
            }
        }
    
        match mode{
            Some(flag) =>{
                match flag.as_str(){
                    "I" | "--install" => install(flags, args, home_dir), 
                    "R" | "--remove" => remove(flags, args, home_dir),
                    "S" | "--sync" => sync(flags, args, home_dir),
                    "Q" | "--query" => query(flags, args, home_dir),
                    "E" | "--edit" => edit(flags, args, home_dir),
                    _ => (),
                }
            }
            None =>{
                eprintln!(
                    "{} Expected operation!",
                    "[!!!]".red()
                );
                util::help_menu();
    
                std::process::exit(1);
            }
        }
    }
    else{
        eprintln!(
            "{} Expected arguments!",
            "[!!!]".red()
        );
        util::help_menu();

        std::process::exit(1);       
    }
}

fn main(){
    let home_dir = dirs::home_dir()
        .unwrap()
        .into_os_string()
        .into_string()
        .unwrap() + "/.config/alps/";

    fs::create_dir_all(&home_dir);
    parser(&home_dir);
}