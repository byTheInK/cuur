use clap::Parser;
use os_info::get as os_get;
use serde_json::from_str as parse_json;
use serde_yaml::from_str as parse_yaml;
use std::fs;
use toml::from_str as parse_toml;

pub mod cli_opts;
pub mod package_managers;

mod lib {
    pub mod funcs;
    pub mod structs;
}

fn main() {
    let args: cli_opts::Args = cli_opts::Args::parse();

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
    let mut is_allowed = lib::funcs::is_os_allowed(
        &os_name,
        works_on,
        package_managers::get_linux,
        package_managers::get_bsd,
        package_managers::get_package_manager_install,
    );
    let is_unknown: bool;

    if args.debug {
        dbg!(&mut is_allowed);
        dbg!(&os_name);
    }

    if os_name == "Unknown" {
        println!("Your system doesn't support Cuur. Please check supported OS at: https://crates.io/crates/os_info.");
        println!("Do you still want to continue? [y]es/[N]o");

        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input.");

        if input.trim().eq_ignore_ascii_case("y") {
            is_allowed = true;
            is_unknown = true;
        }
    }

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
            lib::funcs::execute_commands(exec);
        }

        if let Some(ref update) = startup.update {
            if *update {
                lib::funcs::handle_system_update(
                    &os_name,
                    parsed.sys.pkg_manager.clone(),
                    package_managers::get_package_manager_upgrade,
                );
            }
        }
    }

    println!("Activating the script...");
    let default_aur = parsed.sys.default_aur.unwrap_or(false);

    lib::funcs::handle_package_installation(
        &os_name,
        &aur_helper,
        default_aur,
        parsed.pkg.install,
        parsed.sys.pkg_name.clone(),
        parsed.sys.pkg_manager.clone(),
        package_managers::get_package_manager_install,
    );

    lib::funcs::handle_package_removal(
        &os_name,
        default_aur,
        parsed.pkg.remove,
        parsed.sys.pkg_name.clone(),
        parsed.sys.pkg_manager.clone(),
        package_managers::get_package_manager_remove,
    );
}
