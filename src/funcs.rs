use std::process::Command;
use std::collections::HashMap;

pub fn is_os_allowed(
    os_name: &str,
    works_on: &[String],
    get_linux: fn() -> Vec<String>,
    get_bsd: fn() -> Vec<String>,
    get_package_manager_install: fn(&str) -> Option<&'static str>,
) -> bool {
    if works_on.first().map_or(false, |s| s == "all") {
        return true;
    }

    let includepm: Vec<String> = works_on
        .iter()
        .filter_map(|s| s.strip_prefix("includepm").map(|v| v.trim().to_string()))
        .collect();

    let excludepm: Vec<String> = works_on
        .iter()
        .filter_map(|s| s.strip_prefix("excludepm").map(|v| v.trim().to_string()))
        .collect();

    if !includepm.is_empty() {
        if let Some(pm) = get_package_manager_install(os_name) {
            if !includepm.contains(&pm.to_string()) {
                return false;
            }
        } else {
            return false;
        }
    }

    if !excludepm.is_empty() {
        if let Some(pm) = get_package_manager_install(os_name) {
            if excludepm.contains(&pm.to_string()) {
                return false;
            }
        }
    }

    if works_on.contains(&os_name.to_string()) && !works_on.contains(&"exclude".to_string()) {
        return true;
    }

    if works_on.contains(&"linux".to_string())
        && get_linux().contains(&os_name.to_string())
        && !works_on.contains(&"exclude".to_string())
    {
        return true;
    }

    if works_on.contains(&"bsd".to_string())
        && get_bsd().contains(&os_name.to_string())
        && !works_on.contains(&"exclude".to_string())
    {
        return true;
    }

    false
}

pub fn execute_commands(exec_commands: &[String]) {
    for exec_command in exec_commands {
        println!("Running: {}", exec_command);

        let status = Command::new("sh")
            .arg("-c")
            .arg(exec_command)
            .stdin(std::process::Stdio::inherit())
            .stdout(std::process::Stdio::inherit())
            .stderr(std::process::Stdio::inherit())
            .status();

        match status {
            Ok(s) if s.success() => {
                println!("Command succeeded");
            }
            Ok(s) => {
                eprintln!("Command exited with status: {}", s);
            }
            Err(e) => {
                eprintln!("Failed to execute command: {}", e);
            }
        }
    }
}

pub fn handle_package_installation(
    os_name: &str,
    install_packages: Option<Vec<String>>,
    pkg_name: Option<HashMap<String, HashMap<String, String>>>,
    get_package_manager_install: fn(&str) -> Option<&'static str>,
) {
    if let Some(command_base) = get_package_manager_install(os_name) {
        if let Some(packages) = install_packages {
            if packages.is_empty() {
                eprintln!("No packages to install.");
                return;
            }

            for mut pkg in packages {
                if let Some(ref pkg_map) = pkg_name {
                    if let Some(replacements) = pkg_map.get(os_name) {
                        if let Some(new_pkg) = replacements.get(&pkg) {
                            pkg = new_pkg.clone();
                        }
                    }
                }

                execute_commands(&[format!("{} {}", command_base, pkg)]);
            }
        }
    } else {
        eprintln!("No package manager found for {}", os_name);
    }
}

pub fn handle_package_update(
    os_name: &str,
    get_package_manager_update: fn(&str) -> Option<&'static str>,
) {
    if let Some(command_base) = get_package_manager_update(os_name) {
        let command_string = command_base.to_string();
        execute_commands(&[command_string]);
    } else {
        eprintln!("No update command found for {}", os_name);
    }
}

pub fn handle_package_removal(
    os_name: &str,
    remove_packages: Option<Vec<String>>,
    pkg_name: Option<HashMap<String, HashMap<String, String>>>,
    get_package_manager_remove: fn(&str) -> Option<&'static str>,
) {
    if let Some(command_base) = get_package_manager_remove(os_name) {
        if let Some(packages) = remove_packages {
            if packages.is_empty() {
                eprintln!("No packages to remove.");
                return;
            }

            for mut pkg in packages {
                if let Some(ref pkg_map) = pkg_name {
                    if let Some(replacements) = pkg_map.get(os_name) {
                        if let Some(new_pkg) = replacements.get(&pkg) {
                            pkg = new_pkg.clone();
                        }
                    }
                }

                execute_commands(&[format!("{} {}", command_base, pkg)]);
            }
        }
    } else {
        eprintln!("No package manager found for {}", os_name);
    }
}
