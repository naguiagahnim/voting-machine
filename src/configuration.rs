use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Configuration {
    #[arg(short = 'c', long, value_delimiter = ',', num_args = 1..)]
    pub candidates: Vec<String>,
}

impl Configuration {
    pub fn new() -> Self {
        let mut config = Self::parse();
        config
    }
}
