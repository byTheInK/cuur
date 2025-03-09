use std::collections::HashMap;
use std::process::Command;

pub fn handle_package_installation(
    os_name: &str,
    aur_helper: &str,
    default_aur: bool,
    install_packages: Option<Vec<String>>,
    pkg_name: Option<HashMap<String, HashMap<String, String>>>,
    pkg_manager: Option<HashMap<String, String>>,
    get_package_manager_install: fn(&str) -> Option<(&'static str, &'static str, &'static str)>,
) {
    if let Some((mut pm, prefix, auto_confirm)) = get_package_manager_install(os_name) {
        if let Some(ref pkg_manager_map) = pkg_manager {
            if let Some(replacement) = pkg_manager_map.get(os_name) {
                pm = replacement;
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

pub fn handle_package_removal(
    os_name: &str,
    default_aur: bool,
    remove_packages: Option<Vec<String>>,
    pkg_name: Option<HashMap<String, HashMap<String, String>>>,
    pkg_manager: Option<HashMap<String, String>>,
    get_package_manager_remove: fn(&str) -> Option<(&'static str, &'static str, &'static str)>,
) {
    if let Some((mut pm, prefix, auto_confirm)) = get_package_manager_remove(os_name) {
        if let Some(ref pkg_manager_map) = pkg_manager {
            if let Some(replacement) = pkg_manager_map.get(os_name) {
                pm = replacement;
            }
        }

        if default_aur {
            pm = "yay";
        }

        if let Some(packages) = remove_packages {
            if packages.is_empty() {
                eprintln!("No packages to remove.");
            } else {
                for mut pkg in packages {
                    if let Some(ref pkg_map) = pkg_name {
                        if let Some(replacements) = pkg_map.get(os_name) {
                            if let Some(new_pkg) = replacements.get(&pkg) {
                                pkg = new_pkg.clone();
                            }
                        }
                    }

                    let output = Command::new(pm).args([prefix, auto_confirm, &pkg]).output();

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

pub fn handle_system_update(
    os_name: &str,
    pkg_manager: Option<HashMap<String, String>>,
    get_package_manager_upgrade: fn(&str) -> Option<(&'static str, &'static str, &'static str)>,
) {
    if let Some((mut pm, prefix, auto_confirm)) = get_package_manager_upgrade(os_name) {
        if let Some(ref pkg_manager_map) = pkg_manager {
            if let Some(replacement) = pkg_manager_map.get(os_name) {
                pm = replacement;
            }
        }

        let output = Command::new("sudo")
            .args([pm, prefix, auto_confirm])
            .output();

        match output {
            Ok(res) if res.status.success() => {
                println!("System updated successfully.");
            }
            Ok(res) => {
                eprintln!("Error updating system.");
                eprintln!("{}", String::from_utf8_lossy(&res.stderr));
            }
            Err(e) => {
                eprintln!("Failed to execute command: {}", e);
            }
        }
    } else {
        eprintln!("No package manager found for {}", os_name);
    }
}

pub fn is_os_allowed(
    os_name: &str,
    works_on: &[String],
    get_linux: fn() -> Vec<String>,
    get_bsd: fn() -> Vec<String>,
    get_package_manager_install: fn(&str) -> Option<(&'static str, &'static str, &'static str)>,
) -> bool {
    if works_on.first().map(|s| s == "all").unwrap_or(false) {
        return true;
    }

    let includepm: Vec<String> = works_on
        .iter()
        .filter(|&s| s.starts_with("includepm"))
        .map(|s| s.replace("includepm", "").trim().to_string())
        .collect();

    let excludepm: Vec<String> = works_on
        .iter()
        .filter(|&s| s.starts_with("excludepm"))
        .map(|s| s.replace("excludepm", "").trim().to_string())
        .collect();

    if !includepm.is_empty() {
        let pm = get_package_manager_install(os_name);
        if let Some((pm_name, _, _)) = pm {
            if !includepm.contains(&pm_name.to_string()) {
                return false;
            }
        } else {
            return false;
        }
    }

    if !excludepm.is_empty() {
        let pm = get_package_manager_install(os_name);
        if let Some((pm_name, _, _)) = pm {
            if excludepm.contains(&pm_name.to_string()) {
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
        let output = Command::new("sh").arg("-c").arg(exec_command).output();

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
