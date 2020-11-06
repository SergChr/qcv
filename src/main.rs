mod cli;
mod parser;
#[path="utils/logger.rs"]
mod utils;

extern crate regex;

fn main() {
    cli::init();
}

// WAS DONE:
// 1. Improved "simple" theme
// 2. Added "dark" theme (rename it)
// 3. Some fixes in code
// TODO:
// 1. Finish/improve "dark" theme: make good to print
// 2. Improve README by adding list of available themes(and screenshots)
// 3. Write message when script finishes writing HTML file