use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Cuur {
    pub sys: TomlSystem,
    pub pkg: TomlPackages,
}

#[derive(Deserialize, Debug)]
pub struct TomlSystem {
    pub works_on: Vec<String>,
}

#[derive(Deserialize, Debug)]
pub struct TomlPackages {
    pub install: Option<Vec<String>>,
}
