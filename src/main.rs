use os_info::get as os_get;
use std::fs;
use std::process::Command;
use toml;

pub mod package_managers;
pub mod structs;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        eprintln!("Please enter a file name");
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

    let is_include = parsed.sys.works_on.first() == Some(&"include".to_string());
    let is_exclude = parsed.sys.works_on.first() == Some(&"exclude".to_string());

    let mut is_allowed = parsed.sys.works_on.first() == Some(&"all".to_string());
    let mut install_prefix = "install";

    if parsed.sys.works_on.contains(&os_name) && !is_allowed {
        if is_exclude {
            is_allowed = false;
        }
        if is_include {
            is_allowed = true;
        }
        if !is_exclude && !is_include {
            is_allowed = true;
        }
    }

    if !is_allowed {
        eprintln!("This script does not work in your system. If you wrote an uncorrect name check https://crates.io/crates/os_info.");
        return;
    }

    println!("Activating the script");

    if os_name.clone() == String::from("Arch Linux") {
        install_prefix = "-S";
    }

    match package_managers::get_package_manager(&os_name.as_str()) {
        Some(pm) => {
            if let Some(packages) = &parsed.pkg.install {
                for pkg in packages {
                    let output = Command::new("sudo")
                        .args([pm, install_prefix, "--noconfirm", pkg])
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
                            eprintln!("Failed to run command: {}", e);
                        }
                    }
                }
            } else {
                eprintln!("No packages to install.");
            }
        }
        None => {
            eprintln!("No package manager found for {}", os_name);
            return;
        }
    }
}
