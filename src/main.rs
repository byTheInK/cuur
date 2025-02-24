use os_info::get as os_get;
use std::collections::HashMap;
use std::fs;
use std::process::Command;
use toml::from_str as parse_toml;

pub mod package_managers;
pub mod structs;

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

fn handle_package_installation(
    os_name: &str,
    aur_helper: &str,
    default_aur: bool,
    install_packages: Option<Vec<String>>,
    pkg_name: Option<HashMap<String, HashMap<String, String>>>,
    pkg_manager: Option<HashMap<String, HashMap<String, String>>>,
) {
    if let Some((mut pm, prefix, auto_confirm)) =
        package_managers::get_package_manager_install(os_name)
    {
        if let Some(ref pkg_manager_map) = pkg_manager {
            if let Some(replacement) = pkg_manager_map.get(os_name) {
                if let Some(new_pm) = replacement.get("install") {
                    pm = new_pm;
                }
            }
        }

        if default_aur {
            pm = aur_helper;
        }

        if let Some(packages) = install_packages {
            if packages.is_empty() {
                eprintln!("No packages to install.");
            } else {
                for mut pkg in packages {
                    // Check for pkg_name replacement
                    if let Some(ref pkg_map) = pkg_name {
                        if let Some(replacements) = pkg_map.get(os_name) {
                            if let Some(new_pkg) = replacements.get(&pkg) {
                                pkg = new_pkg.clone();
                            }
                        }
                    }

                    let output = Command::new("sudo")
                        .args([pm, prefix, auto_confirm, &pkg])
                        .output();

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
}




fn handle_package_removal(
    os_name: &str,
    default_aur: bool,
    remove_packages: Option<Vec<String>>,
    pkg_name: Option<HashMap<String, HashMap<String, String>>>,
    pkg_manager: Option<HashMap<String, HashMap<String, String>>>,
) {
    if let Some((mut pm, prefix, auto_confirm)) =
        package_managers::get_package_manager_remove(os_name)
    {
        // Check for pkg_manager replacement
        if let Some(ref pkg_manager_map) = pkg_manager {
            if let Some(replacement) = pkg_manager_map.get(os_name) {
                if let Some(new_pm) = replacement.get("remove") {
                    pm = new_pm; // Replace the package manager for removal
                }
            }
        }

        if default_aur {
            pm = "yay"; // Using yay as the default AUR helper
        }

        if let Some(packages) = remove_packages {
            if packages.is_empty() {
                eprintln!("No packages to remove.");
            } else {
                for mut pkg in packages {
                    // Check for pkg_name replacement
                    if let Some(ref pkg_map) = pkg_name {
                        if let Some(replacements) = pkg_map.get(os_name) {
                            if let Some(new_pkg) = replacements.get(&pkg) {
                                pkg = new_pkg.clone();
                            }
                        }
                    }

                    // Run the removal command
                    let output = Command::new(pm)
                        .args([prefix, auto_confirm, &pkg])
                        .output();

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

    let parsed: structs::Cuur = match parse_toml(&contents) {
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

    handle_package_installation(&os_name, &aur_helper, default_aur, parsed.pkg.install, parsed.sys.pkg_name.clone(), parsed.sys.pkg_manager.clone());

    handle_package_removal(&os_name, default_aur, parsed.pkg.remove, parsed.sys.pkg_name.clone(), parsed.sys.pkg_manager.clone());
}