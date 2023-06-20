# dpm - Data Package Manager

`dpm` is a CLI app for creating **data packages:** software libraries to query specific datasets.

Data packages are best understood from the perspectives of two kinds of user:

- For **data consumers** (app devs, analysts, etc.), data packages are a [database abstraction layer](https://en.wikipedia.org/wiki/Database_abstraction_layer) that gives an excellent developer experience.
  - **Tailored API.** In contrast to a generic query builder, data packages are tailored to the specific dataset being accessed. This is leveraged to implement the maximum possible amount of developer niceties such as autocomplete, type inference, and docs that advertise dataset metadata and caveats.
  - **Agnostic to data location.** You use the same query API regardless of whether the data lives in Patch, Snowflake, S3, or elsewhere.
- For **data maintainers** (data engineers, etc.), data packages are an interface to the dataset that that you maintain. As software packages, they are a versioned artifact that can serve as the collaboration boundary between you and those who depend on your data. New releases of a data package signify schema changes, metadata changes, or other important evolution that you want your dependents to be aware of.

For full docs, see https://docs.dpm.sh/.

## Installation

> :rotating_light: **NOTE:** dpm is alpha-quality software. Breaking changes may occur in any release. We want your feedback! Please file GitHub issues with any feedback that you have.

First, you must have Rust installed. See https://rustup.rs/ for instructions.

Then, clone this repo, enter it, and explore the command tree via `cargo run -- --help`.

For the remainder of the docs, see https://docs.dpm.sh/.
