use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about = "Cuur allows you to install and remove packages in a file while being cross platform. https://github.com/byTheInK/cuur")]
pub struct Args {
    #[arg(value_name = "FILE")]
    pub file: String,

    #[arg(long, conflicts_with = "toml")]
    pub yaml: bool,

    #[arg(long, default_value_t = true, conflicts_with = "yaml")]
    pub toml: bool,
}