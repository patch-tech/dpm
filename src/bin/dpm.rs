use clap::Parser;

use dpm::App;

fn main() {
    let app = App::parse();
    app.exec()
}
