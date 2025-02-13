use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Cuur {
    pub sys: TomlSystem,
}

#[derive(Deserialize, Debug)]
pub struct TomlSystem {
    pub works_on: Vec<String>,
}
