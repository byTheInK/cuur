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
    let is_exclude = works_on.first() == Some(&"exclude".to_string());
    let is_allowed = works_on.first() == Some(&"all".to_string())
        || (works_on.contains(&os_name) && !is_exclude);

    let aur_helper = if let Some(ref helper) = parsed.sys.aur_helper {
        helper.clone()
    } else {
        String::from("yay")
    };

    if !is_allowed {
        eprintln!(
            "This script does not support your system. If you wrote an incorrect name, check: https://crates.io/crates/os_info."
        );
        return;
    }

    if let Some(ref startup) = parsed.startup {
        if let Some(ref exec) = startup.exec {
            if exec.is_empty() {
                println!("Nothing to execute.");
            }
            for exec_command in exec {
                let full_chunk: Vec<&str> = exec_command.split_whitespace().collect();

                let output = Command::new(full_chunk[0]).args(&full_chunk[1..]).output();

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
    }

    println!("Activating the script...");
    let default_aur = parsed.sys.default_aur.unwrap_or(false);

    if let Some((mut pm, prefix, auto_confirm)) =
        package_managers::get_package_manager_install(&os_name)
    {
        if default_aur {
            pm = aur_helper.as_str();
        }

        if let Some(packages) = &parsed.pkg.install {
            if packages.is_empty() {
                eprintln!("No packages to install.");
            } else {
                for pkg in packages {
                    let output = Command::new(pm).args([prefix, auto_confirm, pkg]).output();

                    match output {
                        Ok(res) if res.status.success() => {
                            println!("Package {} installed successfully.", pkg);
                        }
                        Ok(res) => {
                            eprintln!("Error installing package: {}", pkg);
                            eprintln!("{}", String::from_utf8_lossy(&res.stderr));
                        }
                        Err(e) => {
                            eprintln!("Failed to execute command: {}", e);
                        }
                    }
                }
            }
        }
    } else {
        eprintln!("No package manager found for {}", os_name);
    }

    if let Some((mut pm, prefix, auto_confirm)) =
        package_managers::get_package_manager_remove(&os_name)
    {
        if default_aur {
            pm = "yay";
        }

        if let Some(packages) = &parsed.pkg.remove {
            if packages.is_empty() {
                eprintln!("No packages to remove.");
            } else {
                for pkg in packages {
                    let output = Command::new(pm).args([prefix, auto_confirm, pkg]).output();

                    match output {
                        Ok(res) if res.status.success() => {
                            println!("Removed package {} successfully.", pkg);
                        }
                        Ok(res) => {
                            eprintln!("Error removing package: {}", pkg);
                            eprintln!("{}", String::from_utf8_lossy(&res.stderr));
                        }
                        Err(e) => {
                            eprintln!("Failed to execute command: {}", e);
                        }
                    }
                }
            }
        }
    } else {
        eprintln!("No package manager found for {}", os_name);
    }
}
