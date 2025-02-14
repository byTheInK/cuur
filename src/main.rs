use os_info::get as os_get;
use std::fs;
use std::process::Command;
use toml;

pub mod package_managers;
pub mod structs;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        eprintln!("Please enter a file name.");
        return;
    }

    let contents = match fs::read_to_string(&args[1]) {
        Ok(content) => content,
        Err(err) => {
            eprintln!("Error reading file: {}", err);
            return;
        }
    };

    let parsed: structs::Cuur = match toml::from_str(&contents) {
        Ok(config) => config,
        Err(err) => {
            eprintln!("Error parsing TOML: {}", err);
            return;
        }
    };

    let os_name = os_get().os_type().to_string();

    let works_on = &parsed.sys.works_on;
    let is_include = works_on.first() == Some(&"include".to_string());
    let is_exclude = works_on.first() == Some(&"exclude".to_string());

    let mut is_allowed = works_on.first() == Some(&"all".to_string());

    let default_aur = &parsed.sys.default_aur;

    if works_on.contains(&os_name) && !is_allowed {
        is_allowed = !is_exclude || is_include;
    }

    if !is_allowed {
        eprintln!(
            "This script does not support your system. If you wrote an incorrect name, check: https://crates.io/crates/os_info."
        );
        return;
    }

    println!("Activating the script...");

    dbg!(&parsed);
    match package_managers::get_package_manager_install(&os_name) {
        Some((mut pm, prefix, auto_confirm)) => {
            dbg!(default_aur.unwrap_or(false));
            if default_aur.unwrap_or(false) {
                pm = "yay";
                dbg!(pm);
            }

            dbg!(pm);

            if let Some(packages) = &parsed.pkg.install {
                if packages.is_empty() {
                    eprintln!("No packages to install.");
                    return;
                }

                for pkg in packages {
                    let output = Command::new("sudo")
                        .args([pm, prefix, auto_confirm, pkg])
                        .output();

                    match output {
                        Ok(res) => {
                            if !res.status.success() {
                                eprintln!("Error installing package: {}", pkg);
                                eprintln!("{}", String::from_utf8_lossy(&res.stderr));
                            } else {
                                println!("Package {} installed successfully.", pkg);
                            }
                        }
                        Err(e) => {
                            eprintln!("Failed to execute command: {}", e);
                        }
                    }
                }
            } else {
                eprintln!("No packages to install.");
            }
        }
        None => {
            eprintln!("No package manager found for {}", os_name);
        }
    }

    match package_managers::get_package_manager_remove(&os_name) {
        Some((mut pm, prefix, auto_confirm)) => {
            if default_aur.unwrap_or(false) {
                pm = "yay";
            }

            if let Some(packages) = &parsed.pkg.remove {
                if packages.is_empty() {
                    eprintln!("No packages to install.");
                    return;
                }

                for pkg in packages {
                    let output = Command::new("sudo")
                        .args([pm, prefix, auto_confirm, pkg])
                        .output();

                    match output {
                        Ok(res) => {
                            if !res.status.success() {
                                eprintln!("Error removing package: {}", pkg);
                                eprintln!("{}", String::from_utf8_lossy(&res.stderr));
                            } else {
                                println!("Removed package {} successfully.", pkg);
                            }
                        }
                        Err(e) => {
                            eprintln!("Failed to execute command: {}", e);
                        }
                    }
                }
            } else {
                eprintln!("No packages to remove.");
            }
        }
        None => {
            eprintln!("No package manager found for {}", os_name);
        }
    }
}
