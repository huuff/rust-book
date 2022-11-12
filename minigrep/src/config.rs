use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about=None)]
pub struct Config {
    #[arg(short, long)]
    pub query: String,
    #[arg(short, long)]
    pub file_path: String,
    #[arg(short, long)]
    pub ignore_case: bool,
}

