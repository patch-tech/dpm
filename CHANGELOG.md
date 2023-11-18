# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- descriptor: Support new `accelerated` field at the top level of the descriptor. If set while running `publish`, the resulting version will be an accelerated version. Accelerated packages have a certain lifecycle, which stderr output will explain.
- `source create bigquery`: Support creating BigQuery sources
- `package list`: Support listing packages usable (buildable, queryable) by the logged-in user. List is presented in a table.

### Changed
- `init`: Make source name a named argument (`-s`/`--source`). Change long form of `--package-name` to `--package`.
- `build-package`: Update static code in all targets to redefine the
`inPast` boolean operator in terms of computed relative time bounds.
- `build-package`: Update `dpm_agent.proto` to include support for table joins. Update generated protobuf code in all targets.
- `build-package`: Update static code in all targets to support the following temporal projection operators:
  - `week` - project to week of year;
  - `dayOfWeek` - project to day of week;
  - `date` - project to date part of date-time;
  - `time` - project to time part of date-time.

### Deprecated

### Removed

### Fixed

### Security

## [0.4.0] - 2023-09-21
### Added
- Add `--staging-database <NAME>` option (default value: `PATCH`) when creating a Snowflake source.

### Changed
- `describe`: Rename `dpm describe` to `dpm init`.
- `describe`: Give helpful error message if introspection metadata during the command is too large.
- `login`: If a session.json exists but contains a token failing a basic validity check, initiate the normal login flow.

## [0.3.0] - 2023-09-15
### Added
- `build-package`: Support building instances both from a descriptor ("draft
packages") and from a reference to a published package ("release packages").

### Fixed
- `build-package`: Generated C# `.csproj` to use `<Version>` tag with `{pkg-semver}-{code-semver}`.
- `build-package`: C# target directory to use correct semver in path.

## [0.2.0] - 2023-09-12
### Added
- Support for C# codegen.
- Define missing `sum` aggregator for supported targets.
- C# codegen upgrades:
  - Define `TimeField` class.
  - Fill in missing temporal comparison operators.
  - Make query execution asynchronous.
  - Support a dynamic return type for query results.

### Fixed
- Naming error in dpm_agent.proto Literal message definition.
- Fix bug that did not alias instances of Field<T> correctly.

## [0.1.0] - 2023-08-18

### Added

- Initial release
