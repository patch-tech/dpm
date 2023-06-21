# dpm – Data Package Manager

`dpm` is a CLI app for creating **data packages:** software libraries to query specific datasets.

Data packages are best understood from the perspectives of two kinds of users:

- For **data consumers** (app devs, analysts, etc.), data packages are a [database abstraction layer](https://en.wikipedia.org/wiki/Database_abstraction_layer) that gives an excellent developer experience.
  - **Tailored API.** In contrast to a generic query builder, data packages are tailored to the specific dataset being accessed. This is leveraged to implement the maximum possible amount of developer niceties such as autocomplete, type hints, and docs that advertise dataset metadata and caveats.
  - **Agnostic to data location.** You use the same query API regardless of whether the data lives in Patch, Snowflake, S3, or elsewhere. This means there's less to learn and less disruption if data moves from one database to another.
- For **data maintainers** (data engineers, etc.), data packages are an interface to the dataset that you maintain. As software packages, they are a versioned artifact that can serve as the collaboration boundary between you and those who depend on your data. New releases of a data package signify any important evolution that you want your dependents to be aware of such as schema changes, metadata changes, and changes to the data's location.

For full docs, see https://docs.dpm.sh/.

## Installation

> :rotating_light: **NOTE:** dpm is alpha-quality software. Breaking changes may occur in any release. We want your feedback! Please file GitHub issues with any feedback that you have, or start a GitHub discussion if you have questions about usage or best practices.

First, you must have Rust installed. See https://rustup.rs/ for instructions.

Then,

1. Clone this repo and `cd` into it.
2. Run `cargo install --path .` to compile and install `dpm`.
3. Explore the CLI starting with `dpm --help`.

To uninstall, `cargo uninstall --bin dpm`.

For the remainder of the docs, see https://docs.dpm.sh/.
