use std::process::Command;
use std::collections::HashMap;

pub fn is_os_allowed(
    os_name: &str,
    works_on: &[String],
    get_linux: fn() -> Vec<String>,
    get_bsd: fn() -> Vec<String>,
    get_package_manager_install: fn(&str) -> Option<&'static str>,
) -> bool {

    // Allow if the first argument is all
    if works_on.first().map_or(false, |s| s == "all") {
        return true;
    }

    // Check if it says include package manager
    let includepm: Vec<String> = works_on
        .iter()
        .filter_map(|s| s.strip_prefix("includepm").map(|v| v.trim().to_string()))
        .collect();

    // Check if it says exclude package manager
    let excludepm: Vec<String> = works_on
        .iter()
        .filter_map(|s| s.strip_prefix("excludepm").map(|v| v.trim().to_string()))
        .collect();

    // Checks if include package manager is not empty
    if !includepm.is_empty() {
        if let Some(pm) = get_package_manager_install(os_name) {
            if !includepm.contains(&pm.to_string()) {
                return false;
            }
        } else {
            return false;
        }
    }

    // Checks if exclude package manager is not empty
    if !excludepm.is_empty() {
        if let Some(pm) = get_package_manager_install(os_name) {
            if excludepm.contains(&pm.to_string()) {
                return false;
            }
        }
    }


    // Check if the os name is in the arguments
    if works_on.contains(&os_name.to_string()) && !works_on.contains(&"exclude".to_string()) {
        return true;
    }

    // Checks if it contains linux and the os name is in the linux category
    if works_on.contains(&"linux".to_string())
        && get_linux().contains(&os_name.to_string())
        && !works_on.contains(&"exclude".to_string())
    {
        return true;
    }

    // Checks if it contains bsd and the os name is in the bsd category
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

        // Run the command with sh -c
        // Give stdin, stdout, stderr to make it interactive
        let status = Command::new("sh")
            .arg("-c")
            .arg(exec_command)
            .stdin(std::process::Stdio::inherit())
            .stdout(std::process::Stdio::inherit())
            .stderr(std::process::Stdio::inherit())
            .status();

        // Checking the status of the command
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
    // Gets the installation command from the command given (get_package_manager_install)
    if let Some(command_base) = get_package_manager_install(os_name) {
        // Gets the packages
        if let Some(packages) = install_packages {
            // Cheks if there any packages
            if packages.is_empty() {
                eprintln!("No packages to install.");
                return;
            }

            // Iterate over packages
            for mut pkg in packages {
                // Get the package HashMap
                if let Some(ref pkg_map) = pkg_name {
                    // Looks at the os name and gets that name from package HashMap
                    if let Some(replacements) = pkg_map.get(os_name) {
                        // Gets the package from package HashMap
                        if let Some(new_pkg) = replacements.get(&pkg) {
                            pkg = new_pkg.clone();
                        }
                    }
                }

                // Starts executing the installation command
                execute_commands(&[format!("{} {}", command_base, pkg)]);
            }
        }
    } else {
        // Errors if it doesn't find the package manager of the os
        eprintln!("No package manager found for {}", os_name);
    }
}

pub fn handle_package_update(
    os_name: &str,
    get_package_manager_update: fn(&str) -> Option<&'static str>,
) {
    // Gets the update command from the command given (get_package_manager_update)
    if let Some(command_base) = get_package_manager_update(os_name) {
        // Converts the command to String and updates the system with that command
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
    // Gets the remove command from the command given (get_package_manager_remove)
    if let Some(command_base) = get_package_manager_remove(os_name) {
        // Gets the packages
        if let Some(packages) = remove_packages {
            // Checks if the packages are empty
            if packages.is_empty() {
                eprintln!("No packages to remove.");
                return;
            }

            // Iterates over packages
            for mut pkg in packages {
                // Gets the package HashMap
                if let Some(ref pkg_map) = pkg_name {
                    // Looks at the os name and gets that name from package HashMap
                    if let Some(replacements) = pkg_map.get(os_name) {
                        // Gets the package
                        if let Some(new_pkg) = replacements.get(&pkg) {
                            pkg = new_pkg.clone();
                        }
                    }
                }

                // Starts the removal process
                execute_commands(&[format!("{} {}", command_base, pkg)]);
            }
        }
    } else {
        // Errors if it doesnt find the package manager
        eprintln!("No package manager found for {}", os_name);
    }
}
