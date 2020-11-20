mod cli;
mod parser;
#[path="utils/logger.rs"]
mod utils;

extern crate regex;

fn main() {
    cli::init();
}
