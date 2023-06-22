// Bring this trait into scope so that references to `Parser::parse` resolve.
use clap::{Command, CommandFactory, Parser};
use clap_complete::{self, generate};
use std::io;

use dpm::App;

fn print_completions<G: clap_complete::Generator>(gen: G, cmd: &mut Command) {
    generate(gen, cmd, cmd.get_name().to_string(), &mut io::stdout());
}

#[tokio::main]
async fn main() {
    let app = App::parse();

    if let Some(generator) = app.generator {
        let mut cmd = App::command();
        eprintln!("Generating completion file for {generator:?}...");
        print_completions(generator, &mut cmd);
    } else {
        println!("{app:#?}");
    }

    app.exec().await
}
