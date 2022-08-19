use crate::sift;
use crate::util;
use std::{
    fs,
    path::Path,
    process::{Command, Stdio},
    io::ErrorKind,
};
use colored::Colorize;

pub fn install_help(){
    println!("{} {{-I}} [flag] [...]", "usage:".white());
    println!("{}", "flags:".white());
    println!("   {{-g --group}} [group(s)] : install group to config");
    println!("   {{-c --config}} [group] [config(s)] : install file to group");
    println!("   {{-p --package}} [group] [package(s)] : install package to group");
    println!("   {{-s --script}} [group] [script(s)] : install script to group");
}

pub fn install_group(mut args: Vec<String>, home_dir: &str){
    sift::missing_args(&mut args, 1);
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

pub fn install_package(mut args: Vec<String>, home_dir: &str){
    let mut group = String::new();

    sift::missing_group(home_dir, &mut args, &mut group);
    sift::missing_args(&mut args, 1);
    sift::invalid_packages(home_dir, &mut args, true, &group);

    for arg in args{
        util::config_write(&group, "[PACKAGES]", &arg, home_dir, true);

        println!(
            "{} Installed {}/{}/{}",
            "[+]".green(),
            group.green(),
            "packages".green(),
            arg.green()
        );
    }

}

pub fn install_config(mut args: Vec<String>, home_dir: &str){
    let mut group = String::new();

    sift::missing_group(home_dir, &mut args, &mut group);
    sift::missing_args(&mut args, 1);
    sift::invalid_configs(home_dir, &mut args, true, &group);

    fs::create_dir(home_dir.to_owned() + &group + "/configs");

    for arg in &mut args{
        let mut config = arg.to_owned();
        util::to_template(&mut config);

        let (_, config_name) = arg.rsplit_once('/').unwrap();
        let postfix = "_".to_owned()
            + &(util::dup_count(config_name, &group, home_dir)+1).to_string();
        config = config + &postfix;

        util::copy_dir(
            &arg, 
            home_dir.to_owned()
                + &group
                + "/configs/"
                + config_name
                + &postfix
        );
        util::config_write(&group, "[CONFIGS]", &config, home_dir, true);

        println!(
            "{} Installed {}/{}/{}",
            "[+]".green(),
            group.green(),
            "configs".green(),
            config_name.green()
        );
    }
}

pub fn install_script(mut args: Vec<String>, home_dir: &str){
    let mut group = String::new();

    sift::missing_group(home_dir, &mut args, &mut group);
    sift::missing_args(&mut args, 1);
    sift::invalid_scripts(home_dir, &mut args, true, &group);

    fs::create_dir(home_dir.to_owned() + &group + "/scripts");
    
    for arg in &args{
        let arg_name = arg.split('/').last().unwrap();
        util::config_write(&group, "[SCRIPTS]", arg_name, home_dir, true);

        fs::copy(
            &arg, 
            home_dir.to_owned()
                + &group
                + "/scripts/"
                + arg_name
        );

        println!(
            "{} Installed {}/{}/{}",
            "[+]".green(),
            group.green(),
            "scripts".green(),
            arg_name.green()
        );
    }
}

pub fn remove_help(){
    println!("{} {{-R}} [flag]", "usage:".white());
    println!("{}", "flags:".white());
    println!("   {{-g --group}} [group(s)] : remove specified groups including contents");
    println!("   {{-c --config}} [group] [config(s)] : remove specified config(s) of group");       
    println!("   {{-p --package}} [group] [package(s)] : remove specified package(s) of group");
    println!("   {{-s --script}} [group] [script(s)] : remove specified script(s) of group");       
}

pub fn remove_group(mut args: Vec<String>, home_dir: &str){
    sift::missing_args(&mut args, 1);
    sift::invalid_groups(home_dir, &mut args, false);

    for arg in args{
        if fs::remove_dir_all(home_dir.to_owned() + &arg).is_ok(){
            println!(
                "{} Removed group ({})...",
                "[-]".green(),
                arg.green()
            );
        }
    }
}

pub fn remove_package(mut args: Vec<String>, home_dir: &str){
    let mut group = String::new();

    sift::missing_group(home_dir, &mut args, &mut group);
    sift::missing_args(&mut args, 1);
    sift::invalid_packages(home_dir, &mut args, false, &group);

    for arg in &args{
        util::config_write(&group, "[PACKAGES]", arg, home_dir, false);

        println!(
            "{} Removed {}/{}/{}...",
            "[-]".green(),
            group.green(),
            "packages".green(),
            arg.green()
        );
    }
}

pub fn remove_config(mut args: Vec<String>, home_dir: &str){
    let mut group = String::new();

    sift::missing_group(home_dir, &mut args, &mut group);
    sift::missing_args(&mut args, 1);
    sift::invalid_configs(home_dir, &mut args, false, &group);


    for arg in &args{
        util::config_write(&group, "[CONFIGS]", arg, home_dir, false);

        let config_name = arg.split('/').last().unwrap();
        let config_path = home_dir.to_owned() + &group + "/configs/" + config_name;

        if Path::new(&arg).is_dir(){
            fs::remove_dir_all(config_path);
        }
        else{
            fs::remove_file(config_path);
        }

        println!(
            "{} Removed {}/{}/{}...",
            "[-]".green(),
            group.green(),
            "configs".green(),
            config_name.green()
        );
    }
}

pub fn remove_script(mut args: Vec<String>, home_dir: &str){
    let mut group = String::new();

    sift::missing_group(home_dir, &mut args, &mut group);
    sift::missing_args(&mut args, 1);
    sift::invalid_scripts(home_dir, &mut args, false, &group);

        for arg in &args{
        util::config_write(&group, "[SCRIPTS]", arg, home_dir, false);
        fs::remove_file(home_dir.to_owned() + &group + "/scripts/" + arg);

        println!(
            "{} Removed {}/{}/{}...",
            "[-]".green(),
            group.green(),
            "scripts".green(),
            arg.green()
        );
    } 
}

pub fn sync_help(){
    println!("{} {{-S}} [flag]", "usage:".white());
    println!("{}", "flags:".white());
    println!("   {{-g --group}} [group] : sync system with all group contents");
    println!("   {{-c --config}} [group] : sync system with only group configs");       
    println!("   {{-p --package}} [group] : sync system with only group packages");
    println!("   {{-s --script}} [group] : sync system with only group scripts"); 
}

pub fn sync_group(home_dir: &str, group: &str){
    sync_package(home_dir, group);
    sync_config(home_dir, group);
    sync_script(home_dir, group);
}

pub fn sync_package(home_dir: &str, group: &str){
    println!(
        "{} Syncing packages of group ({}) {}",
        "=====".purple(),
        group.purple(),
        "=====".purple()
    );

    let mut num_packages = 0;

    let packages = util::read_label("[PACKAGES]", group, home_dir)
        .split_whitespace()
        .filter_map(|package|{
            num_packages += 1;

            let handle = Command::new("pacman")
                .args(["-Sg", package])
                .stdout(Stdio::piped())
                .output()
                .unwrap();

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
            else if handle.status.success(){
                let missing = String::from_utf8(handle.stdout)
                    .unwrap()
                    .split_whitespace()
                    .filter_map(|entry|{
                        if entry != package{
                            Some(entry.to_owned())
                        }
                        else{
                            None
                        }
                    })
                    .any(|entry|{
                        !Command::new("pacman")
                            .args(["-Q", &entry])
                            .output()
                            .unwrap()
                            .status
                            .success()
                });

                if !missing{
                    eprintln!(
                        "{} Package group ({}) already installed to system!",
                        "[!]".yellow(),
                        package.yellow()
                    );
                    None
                }
                else{
                    println!(
                        "{} Installing package group ({}) to system...",
                        "[+]".purple(),
                        package.purple()
                    );
                    
                    Some(package.to_owned())                   
                }
            }
            else{
                let handle_package = Command::new("pacman")
                    .args(["-Ss", &("^".to_owned() + package + "$")])
                    .output()
                    .unwrap();
                
                if handle_package.status.success(){
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
                    util::config_write(group, "[PACKAGES]", package, home_dir, false);

                    None
                }
            }
        }).collect::<Vec<String>>();
    
    if !packages.is_empty(){
        match Command::new("sudo")
            .args(["pacman", "-S"])
            .args(packages.as_slice())
            .status()
        {
            Ok(_) =>{
                println!(
                    "{} Synced ({}/{num_packages}) packages...",
                    "[~]".purple(),
                    packages.len()
                );
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

pub fn sync_config(home_dir: &str, group: &str){
    println!(
        "{} Syncing configs of group ({}) {}",
        "=====".purple(),
        group.purple(),
        "=====".purple()
    );

    let mut num_configs = 0;

    let configs = util::read_label("[CONFIGS]", group, home_dir)
        .split_whitespace()
        .map(|config| config.to_owned() )
        .collect::<Vec<String>>();

    if !configs.is_empty(){
        for config in &configs
        {
            let mut path_dst = config.to_owned();
            if let Some(name) = config.rsplit_once('_'){
                if name.1.parse::<usize>().is_ok(){
                    path_dst = name.0.to_owned();
                }
            }
            util::to_userdir(&mut path_dst);

            let config_name = config.split('/').last().unwrap();
            let path_src = home_dir.to_owned() + group + "/configs/" + config_name;

            if Path::new(&path_src).exists(){
                num_configs += 1;
                util::copy_dir(path_src, path_dst);
    
                println!(
                    "{} Synced config ({})!",
                    "[~]".purple(),
                    config_name.purple(),
                );
            }
            else{
                eprintln!(
                    "{} Contents of config ({}) do not exist!",
                    "[!]".yellow(),
                    config_name.yellow()
                );

                util::config_write(group, "[CONFIGS]", &config, home_dir, false);
            }
        }

        println!(
            "{} Synced ({num_configs}/{}) configs...",
            "[~]".purple(),
            configs.len()
        );
    }
    else{
        eprintln!(
            "{} No configs to sync in group ({})",
            "[!]".yellow(),
            group.yellow()
        );
    }
}

pub fn sync_script(home_dir: &str, group: &str){
    println!(
        "{} Syncing scripts of group ({}) {}",
        "=====".purple(),
        group.purple(),
        "=====".purple()
    );

    let mut num_scripts = 0;

    let scripts = util::read_label("[SCRIPTS]", group, home_dir)
        .split_whitespace()
        .map(|script| script.to_owned() )
        .collect::<Vec<String>>();

    if !scripts.is_empty(){
        for script in &scripts{
            let script_path = home_dir.to_owned() + group + "/scripts/" + script;
            let mut handle = Command::new("/".to_owned() + &script_path);

            println!(
                "{} Running script ({})...",
                "[~]".purple(),
                script.purple()
            );

            match handle.status(){
                Ok(_) => {
                    num_scripts += 1;

                    println!(
                        "{} Successfully ran script ({})...",
                        "[~]".purple(),
                        script.purple()
                    );
                }
                Err(error) => {
                    if error.kind() == ErrorKind::NotFound{
                        eprintln!(
                            "{} Contents of script ({}) do not exist!",
                            "[!]".yellow(),
                            script.yellow()
                        );
                    }
                    else{
                        eprintln!(
                            "{} Script ({}) failed to run!",
                            "[!]".yellow(),
                            script.yellow()
                        );
                    }

                    if Path::new(&script_path).is_dir(){
                        fs::remove_dir_all(script_path);
                    }
                    else if Path::new(&script_path).is_file(){
                        fs::remove_file(script_path);
                    }
                    util::config_write(group, "[SCRIPTS]", script, home_dir, false);
                }
            }
        }

        println!(
            "{} Synced ({num_scripts}/{}) scripts...",
            "[~]".purple(),
            scripts.len()
        );
    }
    else{
        eprintln!(
            "{} No scripts to sync in group ({})",
            "[!]".yellow(),
            group.yellow()
        );
    }
}

pub fn query_help(){
    println!("{} {{-Q}} [flag]", "usage:".white());
    println!("{}", "flags:".white());
    println!("   {{-g --group}} [?group(s)] : query installed group(s)");
    println!("   {{-c --config}} [group] [?config(s)]: query installed configs of a group");       
    println!("   {{-p --package}} [group] [?packages(s)] : query installed packages of a group");
    println!("   {{-s --script}} [group] [?scripts(s)] : query installed scripts of a group");      
    println!("{} ? = optional", "hint:".white());
}

pub fn query_group(args: Vec<String>, home_dir: &str){
    let excludes = vec![
        String::from(".git"), 
        String::from(".."), 
        String::from(".")
    ];

    //Credit Raforawesome (programming God)
    let groups: Vec<fs::DirEntry> = fs::read_dir(home_dir).unwrap()
        .filter(|entry|{
            let name = entry.as_ref()
                .unwrap().file_name()
                .into_string().unwrap();
            let entry = entry.as_ref().unwrap()
                .file_type().unwrap();

            entry.is_dir() && !excludes.contains(&name)
        }).map(|x| x.unwrap()).collect();

    if !groups.is_empty(){
        if !args.is_empty(){
            let mut status = 0;

            for arg in &args{
                let contains = groups.iter()
                    .any(|group|{
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
                    "{} {} :: ({}) packages :: ({}) configs :: ({}) scripts", 
                    "[?]".blue(),
                    group.blue(),
                    util::read_label("[PACKAGES]", &group, home_dir).split_whitespace().count(),
                    util::read_label("[CONFIGS]", &group, home_dir).split_whitespace().count(),
                    util::read_label("[SCRIPTS]", &group, home_dir).split_whitespace().count()
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
    }
}

pub fn query_package(mut args: Vec<String>, home_dir: &str){
    let mut group = String::new();
    sift::missing_group(home_dir, &mut args, &mut group);
    util::find(args, "[PACKAGES]", home_dir, &group, |package| package);
}

pub fn query_config(mut args: Vec<String>, home_dir: &str){
    let mut group = String::new();
    sift::missing_group(home_dir, &mut args, &mut group);
    util::find(args, "[CONFIGS]", home_dir, &group, |config|{
        let x = config.rsplit_once('/').unwrap_or((config, config)).1;

        if let Some(name) = x.rsplit_once('_'){
            if name.1.parse::<usize>().is_ok(){
               name.0
            }
            else{
                x
            }
        }
        else{
            x
        }
    });
}

pub fn query_script(mut args: Vec<String>, home_dir: &str){
    let mut group = String::new();
    sift::missing_group(home_dir, &mut args, &mut group);
    util::find(args, "[SCRIPTS]", home_dir, &group, |script| script);
}

pub fn edit_help(){
    println!("{} {{-E}} [flag]", "usage".white());
    println!("{}", "flags:".white());
    println!("   {{-g --group}} [group(s)] : edit installed group(s)");
    println!("   {{-c --config}} [group] [config(s)]: edit installed configs of a group");       
    println!("   {{-s --script}} [group] [scripts(s)] : edit installed scripts of a group");      
}

pub fn edit_group(mut args: Vec<String>, home_dir: &str, editor: String){
    sift::missing_args(&mut args, 1);

    for arg in &args{
        let config_path = home_dir.to_owned() 
            + arg 
            + "/" 
            + arg 
            + ".conf";

        if Path::new(&config_path).is_file(){
            util::edit_file(&config_path, &editor);
        }
        else{
            eprintln!(
                "{} Group ({}) config does not exist!",
                "[!]".yellow(),
                arg.yellow()
            );
        }
    }
}

pub fn edit_config(mut args: Vec<String>, home_dir: &str, editor: String){
    let mut group = String::new();
    sift::missing_group(home_dir, &mut args, &mut group);
    sift::missing_args(&mut args, 1);

    for arg in &args{
        let config_path = home_dir.to_owned()
            + &group
            + "/configs/"
            + arg;
        let config_name = config_path.split("/configs/")
            .last()
            .unwrap();

        let path = Path::new(&config_path);
        if path.is_file(){
            util::edit_file(&config_path, &editor);
        }
        else if path.is_dir(){
            eprintln!(
                "{} Config ({}) is a directory!",
                "[!]".yellow(),
                config_name.yellow()
            );
        }
        else{
            eprintln!(
                "{} {}/{}/{} does not exist!",
                "[!]".yellow(),
                group.yellow(),
                "configs".yellow(),
                arg.yellow()
            );
        }
    }
}

pub fn edit_script(mut args: Vec<String>, home_dir: &str, editor: String){
    let mut group = String::new();
    sift::missing_group(home_dir, &mut args, &mut group);
    sift::missing_args(&mut args, 1);

    for arg in &args{
        let config_path = home_dir.to_owned()
            + &group
            + "/scripts/"
            + arg;

        if Path::new(&config_path).is_file(){
            util::edit_file(&config_path, &editor);
        }
        else{
            eprintln!(
                "{} {}/{}/{} does not exist!",
                "[!]".yellow(),
                group.yellow(),
                "scripts".yellow(),
                arg.yellow()
            );
        }
    }
}
