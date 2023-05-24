//! Command parsers and logic.

use clap::Parser;

#[derive(Parser)]
#[command(author, version, about)]
pub struct App {
    one: u32,
    two: u32,
}

impl App {
    pub fn exec(self) {
        println!("The sum is {}", self.one + self.two);
    }
}
