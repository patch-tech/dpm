# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- `build-package`: Add new target: C#.
- `build-package`: Add `sum` aggregator to all targets.
- `build-package`: Support building instances both from a descriptor ("draft packages") and from a reference to a published package ("standard packages").

### Changed

### Deprecated

### Removed

### Fixed
- Generated C# `.csproj` to use `<Version>` tag with `{pkg-semver}-{code-semver}`.
- C# target directory to use correct semver in path.

### Security

## [0.1.0] - 2023-08-18

### Added

- Initial release

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
