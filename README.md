# dpm – Data Package Manager

`dpm` is a CLI app for creating **data packages:** software libraries to query specific datasets.

Data packages are best understood from the perspectives of two kinds of users:

- For **data consumers** (app devs, analysts, etc.), data packages are a [database abstraction layer](https://en.wikipedia.org/wiki/Database_abstraction_layer) that gives an excellent developer experience.
  - **Tailored API.** In contrast to a generic query builder, data packages are tailored to the specific dataset being accessed. This is leveraged to implement the maximum possible amount of developer niceties such as autocomplete, type hints, and docs that advertise dataset metadata and caveats.
  - **Agnostic to data location.** You use the same query API regardless of whether the data lives in Patch, Snowflake, S3, or elsewhere. This means there's less to learn and less disruption if data moves from one database to another.
- For **data maintainers** (data engineers, etc.), data packages are an interface to the dataset that you maintain. As software packages, they are a versioned artifact that can serve as the collaboration boundary between you and those who depend on your data. New releases of a data package signify any important evolution that you want your dependents to be aware of such as schema changes, metadata changes, and changes to the data's location.

Installation instructions are below. For full docs, see https://docs.dpm.sh/.

## Installation

> :rotating_light: **NOTE:** dpm is beta-quality software. Breaking changes may occur in any release. We want your feedback! Please file GitHub issues with any feedback that you have, or start a GitHub discussion if you have questions about usage or best practices.

Binaries for some platforms are available on the [Releases](https://github.com/patch-tech/dpm/releases) page. OS-specific instructions are below.

### macOS

Support for installation via Homebrew is coming soon. x86-64 binaries are also available on the [Releases](https://github.com/patch-tech/dpm/releases) page.

### Linux

Homebrew instructions are coming soon. Otherwise, consider building from source, described below.

### Windows

Support for installation via Scoop is coming soon. x86-64 binaries are also available on the [Releases](https://github.com/patch-tech/dpm/releases) page.

### Manually building from source

First, you must have Rust installed. See https://rustup.rs/ for instructions.

Second, you must have the Protobuf compiler, `protoc`, installed. You can find the latest release [here](https://github.com/protocolbuffers/protobuf/releases/latest). It may also be available via your system-specific package manager (e.g., `brew install protobuf` on macOS).

Then,

1. Clone this repo and `cd` into it.
2. Run `cargo install --path .` to compile and install `dpm`.
3. Explore the CLI starting with `dpm --help`.

To uninstall, `cargo uninstall --bin dpm`.

## Environment variables

- `DPM_AGENT_URL` - URL to dpm-agent. TLS will be used if and only if the scheme is `https`. (default: `https://agent.dpm.sh`)
- `DPM_API_URL` - URL to the DPM Cloud backend service. (default: `https://api.dpm.sh`)
- `DPM_AUTH_TOKEN` - DPM Cloud API token. Operations that require authentication will use this value, if set; otherwise you must have completed `dpm login`. (default: none)
