use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about = "Cuur allows you to install and remove packages in a file while being cross platform. https://github.com/byTheInK/cuur")]
pub struct Args {
    #[arg(value_name = "FILE")]
    pub file: String,

    #[arg(long, conflicts_with_all = &["toml", "json"])]
    pub yaml: bool,

    #[arg(long, default_value_t = true, conflicts_with_all = &["json", "yaml"])]
    pub toml: bool,

    #[arg(long, conflicts_with_all = &["toml", "yaml"])]
    pub json: bool,

    #[arg(long, short)]
    pub debug: bool,
}
