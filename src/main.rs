use os_info::get as os_get;
use std::fs;
use toml;

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
    let os_name = os_get().os_type().to_string().to_lowercase();

    if !(parsed.sys.works_on.get(0) == Some(&"all".to_string()))
        && !parsed
            .sys
            .works_on
            .iter()
            .any(|x| x.to_lowercase() == os_name)
    {
        println!("This script does not work in your system. If you wrote an uncorrect name check https://crates.io/crates/os_info.");
    } else {
        println!("This script is supported on your system");
    }
}
