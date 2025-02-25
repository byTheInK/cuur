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
