use clap::Parser;
use os_info::get as os_get;
use serde_json::from_str as parse_json;
use serde_yaml::from_str as parse_yaml;
use std::{fs, process::exit};
use toml::from_str as parse_toml;

pub mod cli;
pub mod funcs;
pub mod structs;
pub mod package_managers;

fn main(){
    let args: cli::Args = cli::Args::parse();

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

    let parsed: structs::Cuur;

    if args.yaml {
        parsed = match parse_yaml(&contents) {
            Ok(config) => config,
            Err(err) => {
                eprintln!("Error parsing YAML: {}", err);
                return;
            }
        };
    } else if args.json {
        parsed = match parse_json(&contents) {
            Ok(config) => config,
            Err(err) => {
                eprintln!("Error parsing TOML: {}", err);
                return;
            }
        };
    } else {
        parsed = match parse_toml(&contents) {
            Ok(config) => config,
            Err(err) => {
                eprintln!("Error parsing TOML: {}", err);
                return;
            }
        };
    }

    if args.debug {
        dbg!(&parsed);
    }

    let os_name = os_get().os_type().to_string();
    let works_on = &parsed.sys.works_on;
    let is_allowed = funcs::is_os_allowed(
        &os_name,
        works_on,
        package_managers::get_linux,
        package_managers::get_bsd,
        package_managers::get_package_manager_install,
    );

    if os_name == "Unknown" || !is_allowed {
        println!("Your operating system is not supported.");
    }
    
    let aur_helper = parsed
        .sys
        .aur_helper
        .clone()
        .unwrap_or_else(|| "yay".to_string());

        if let Some(ref startup) = parsed.startup {
        if let Some(ref exec) = startup.exec {
            funcs::execute_commands(exec);
        }

        if let Some(ref update) = startup.update {
            if *update {
                funcs::handle_package_update(
                    &os_name,
                    package_managers::get_package_manager_update,
                );
            }
        }
    }

    println!("Activating the script...");
    let default_aur = parsed.sys.default_aur.unwrap_or(false);

    funcs::handle_package_installation(
        &os_name,
        parsed.pkg.install,
        parsed.sys.pkg_name.clone(),
        package_managers::get_package_manager_install,
    );

    funcs::handle_package_removal(
        &os_name,
        parsed.pkg.remove,
        parsed.sys.pkg_name.clone(),
        package_managers::get_package_manager_remove,
    );
}
