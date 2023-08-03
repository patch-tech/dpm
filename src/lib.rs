mod api;
mod codegen;
mod command;
mod descriptor;
mod env;
mod github;
mod session;

#[doc(hidden)]
pub use command::App;

pub mod built_info {
    // The file has been placed there by the build script.
    include!(concat!(env!("OUT_DIR"), "/built.rs"));
}
