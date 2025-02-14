use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Cuur {
    pub sys: TomlSystem,
    pub pkg: TomlPackages,
}

#[derive(Deserialize, Debug)]
pub struct TomlSystem {
    pub default_aur: Option<bool>,
    pub works_on: Vec<String>,
}

#[derive(Deserialize, Debug)]
pub struct TomlPackages {
    pub install: Option<Vec<String>>,
    pub remove: Option<Vec<String>>,
}
