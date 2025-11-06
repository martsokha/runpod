# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Initial implementation of Runpod Rust SDK
- Full support for Runpod REST API endpoints
- `Config` and `ConfigBuilder` for client configuration
- Service modules for managing resources:
  - `PodsService` - Manage GPU/CPU pods
  - `EndpointsService` - Manage serverless endpoints
  - `TemplatesService` - Manage pod and serverless templates
  - `VolumesService` - Manage network volumes
  - `RegistryService` - Manage container registry authentication
  - `BillingService` - Retrieve billing information
- Comprehensive type-safe models for all API resources
- Builder pattern support for configuration via `derive_builder`
- Optional `tracing` feature for logging support
- Enum string conversions via `strum` derives
- Two comprehensive examples:
  - `basic_usage.rs` - Basic API usage
  - `manage_endpoints.rs` - Endpoint management
- GitHub Actions CI/CD:
  - Automated testing on multiple platforms (Linux, macOS, Windows)
  - Format checking with `rustfmt`
  - Linting with `clippy`
  - Documentation building
  - Code coverage reporting
- GitHub Actions for automated releases
- Dependabot configuration for dependency updates
- Comprehensive README with usage examples
- CONTRIBUTING.md with contribution guidelines

### Changed
- N/A

### Deprecated
- N/A

### Removed
- N/A

### Fixed
- N/A

### Security
- N/A

## [0.1.0] - YYYY-MM-DD

### Added
- Initial release

[Unreleased]: https://github.com/martsokha/runpod/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/martsokha/runpod/releases/tag/v0.1.0
