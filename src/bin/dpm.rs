// Bring this trait into scope so that references to `Parser::parse` resolve.
use clap::Parser;

use dpm::App;

#[tokio::main]
async fn main() {
    let app = App::parse();
    app.exec().await
}
