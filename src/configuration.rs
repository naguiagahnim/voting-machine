use clap::Parser;
use clap::ValueEnum;

use crate::interfaces::lexicon::Lexicon;

#[derive(Clone, Copy, ValueEnum, Debug)]
pub enum StorageType {
    File,
    Memory,
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Configuration {
    #[arg(short = 'c', long, value_delimiter = ',', num_args = 1..)]
    pub candidates: Vec<String>,
    #[arg(short = 'm', long, value_delimiter = ',', num_args = 1)]
    pub storage: StorageType,
    #[arg(short = 'l', long, value_delimiter = ',', num_args = 1)]
    pub lexicon: Lexicon,
}

impl Configuration {
    pub fn new() -> Self {
        let mut config = Self::parse();
        config
    }
}
