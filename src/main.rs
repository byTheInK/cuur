use clap::Parser;
use os_info::get as os_get;
use std::process::Command;
use toml::from_str as parse_toml;
use serde_yaml::from_str as parse_yaml;
use std::fs;

pub mod package_managers;
pub mod cli_opts;

mod lib {
    pub mod structs;
    pub mod funcs;
}

fn execute_commands(exec_commands: &[String]) {
    for exec_command in exec_commands {
        let output = Command::new("sh")
            .arg("-c")
            .arg(exec_command)
            .output();

        match output {
            Ok(res) if res.status.success() => {
                println!("Executed startup command");
            }
            Ok(res) => {
                eprintln!("{}", String::from_utf8_lossy(&res.stderr));
            }
            Err(e) => {
                eprintln!("Failed to execute command: {}", e);
            }
        }
    }
}
fn main() {
    let args:cli_opts::Args = cli_opts::Args::parse();

    if args.debug {
        dbg!(&args);
    }

    let contents = match fs::read_to_string(&args.file) {
        Ok(content) => content,
        Err(err) => {
            eprintln!("Error reading file: {}", err);
            return;
        }
    };

    let parsed: lib::structs::Cuur;

    if !args.yaml {
        parsed = match parse_toml(&contents) {
            Ok(config) => config,
            Err(err) => {
                eprintln!("Error parsing TOML: {}", err);
                return;
            }
        };
    } else {
        parsed = match parse_yaml(&contents) {
            Ok(config) => config,
            Err(err) => {
                eprintln!("Error parsing YAML: {}", err);
                return;
            }
        };
    }

    if args.debug {
        dbg!(&parsed);
    }

    let os_name = os_get().os_type().to_string();
    let works_on = &parsed.sys.works_on;
    let is_exclude = works_on.first() == Some(&"exclude".to_string());
    let is_allowed = 
        works_on.first() == Some(&"all".to_string())
        || (works_on.contains(&os_name) && !is_exclude)
        || (parsed.sys.works_on.contains(&"linux".to_string()) && package_managers::get_linux().contains(&os_name))
        || (parsed.sys.works_on.contains(&"bsd".to_string()) && package_managers::get_bsd().contains(&os_name));

    let aur_helper = parsed
        .sys
        .aur_helper
        .clone()
        .unwrap_or_else(|| "yay".to_string());

    if !is_allowed {
        eprintln!(
            "This script does not support your system. If you wrote an incorrect name, check: https://crates.io/crates/os_info."
        );
        return;
    }

    if let Some(ref startup) = parsed.startup {
        if let Some(ref exec) = startup.exec {
            execute_commands(exec);
        }
    }

    println!("Activating the script...");
    let default_aur = parsed.sys.default_aur.unwrap_or(false);

    lib::funcs::handle_package_installation(&os_name, &aur_helper, default_aur, parsed.pkg.install, parsed.sys.pkg_name.clone(), parsed.sys.pkg_manager.clone(), package_managers::get_package_manager_install);

    lib::funcs::handle_package_removal(&os_name, default_aur, parsed.pkg.remove, parsed.sys.pkg_name.clone(), parsed.sys.pkg_manager.clone(), package_managers::get_package_manager_remove);
}