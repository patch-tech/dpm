# Development

## Developing

Workflow is the standard Cargo-based workflow: `cargo build`, `cargo test`, etc. The [Cargo book](https://doc.rust-lang.org/stable/cargo/) is a great reference.

## Repo layout

This repo is a Cargo [package](https://doc.rust-lang.org/cargo/appendix/glossary.html#package) containing 2 [targets](https://doc.rust-lang.org/cargo/reference/cargo-targets.html):
- src/bin/dpm.rs, the binary target that is dpm itself
- src/lib.rs, the library target whose sole use is generating docs via `cargo doc`

This and other layout choices follow [Rain's Rust CLI recommendations](https://rust-cli-recommendations.sunshowers.io/). Thanks, @sunshowers!

## Publishing

Publishing to a registry (presumably crates.io) is disabled until dpm is minimally functional.
