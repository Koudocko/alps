mod sift;
mod util;
mod flag;

use std::{
    fs,
    env, 
    collections::HashSet, 
}; 
use colored::Colorize;

fn install(flags: HashSet<char>, args: Vec<String>, home_dir: &str){
    sift::missing_flag(&flags);

    if flags.contains(&'h'){
        flag::install_help();
    } 
    else if flags.contains(&'g'){
        flag::install_group(args, home_dir);
    }
    else if flags.contains(&'p'){ 
        flag::install_package(args, home_dir);
    }
    else if flags.contains(&'c'){
        flag::install_config(args, home_dir);
    }
    else if flags.contains(&'s'){
        flag::install_script(args, home_dir);
    }
    else{
        sift::invalid_flag();
    }
}

fn remove(flags: HashSet<char>, args: Vec<String>, home_dir: &str){
    sift::missing_flag(&flags);

    if flags.contains(&'h'){
        flag::remove_help();
    }
    else if flags.contains(&'g'){
        flag::remove_group(args, home_dir);
    }
    else if flags.contains(&'c'){
        flag::remove_config(args, home_dir);
    }
    else if flags.contains(&'s'){
        flag::remove_script(args, home_dir);
    }
    else if flags.contains(&'p'){
        flag::remove_package(args, home_dir);
    }
    else{
        sift::invalid_flag();
    }
}

fn sync(flags: HashSet<char>, mut args: Vec<String>, home_dir: &str){
    let mut group = String::new();

    sift::missing_flag(&flags);
    sift::missing_group(home_dir, &mut args, &mut group);

    if flags.contains(&'h'){
        flag::sync_help();
    }
    else if flags.contains(&'g'){
        flag::sync_group(home_dir, &group);
    }
    else if flags.contains(&'p'){
        flag::sync_package(home_dir, &group);
    }
    else if flags.contains(&'c'){
        flag::sync_config(home_dir, &group);
    }
    else if flags.contains(&'s'){
        flag::sync_script(home_dir, &group);
    }
    else{
        sift::invalid_flag();
    }
}

fn query(flags: HashSet<char>, args: Vec<String>, home_dir: &str){
    sift::missing_flag(&flags);

    if flags.contains(&'h'){
        flag::query_help();
    }
    else if flags.contains(&'g'){
        flag::query_group(args, home_dir);
    }
    else if flags.contains(&'p'){
        flag::query_package(args, home_dir);
    }
    else if flags.contains(&'c'){
        flag::query_config(args, home_dir);
    }
    else if flags.contains(&'s'){
        flag::query_script(args, home_dir);
    }
    else{
        sift::invalid_flag();
    }
}

fn edit(flags: HashSet<char>, args: Vec<String>, home_dir: &str){
    let mut editor = String::new();
    sift::missing_flag(&flags);
    sift::missing_editor(&mut editor);

    if flags.contains(&'h'){
        flag::edit_help();
    }   
    else if flags.contains(&'g'){
        flag::edit_group(args, home_dir, editor);
    }
    else if flags.contains(&'c'){
        flag::edit_config(args, home_dir, editor);
    }
    else if flags.contains(&'s'){
        flag::edit_script(args, home_dir, editor);
    }
    else{
        sift::invalid_flag();
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
        util::help_menu();

        std::process::exit(1);
    }

    let mut args: Vec<String> = Vec::new();
    let mut flags = HashSet::new();
    let mut mode = None; 

    for arg in &p_args[1..]{
        if arg.as_bytes()[0] as char == '-'{
            for flag in arg.chars().skip(1){
                match flag{
                    'I' | 'R' | 'S' | 'Q' | 'E' =>{
                        if mode == None{
                            mode = Some(flag); 
                        }
                        else{
                            eprintln!(
                                "{} Cannot use more than one operation!",
                                "[!!!]".red()
                            );

                            std::process::exit(1);
                        }
                    }
                    'h' | 'p' | 'c' | 'g' | 's' => {
                        flags.insert(flag);
                    }
                    op =>{ 
                        eprintln!(
                            "{} Invalid operation ({})! (use -h for help)",
                            "[!!!]".red(),
                            op
                        );
                        util::help_menu();

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
        Some('E') => edit(flags, args, home_dir),
        None =>{
            eprintln!(
                "{} Expected operation! (use -h for help)",
                "[!!!]".red()
            );
            util::help_menu();

            std::process::exit(1);
        }
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