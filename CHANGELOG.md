## [0.3.1] - 2026-11-11

### Documentation

- *(readme)* Overhaul documentation layout and installation guide

### Miscellaneous Tasks

- Configure cargo-release workspace metadata and loosen dependency versions
- Bump reusable workflows and enable musl build with id-token permissions
- Add rust-toolchain.toml with stable channel, components, and musl target
## [0.3.0] - 2026-07-20

### Bug Fixes

- Remove expect() panic path in password generation
- Remove unused std::char import
- Remove redundant value_parser attributes from clap args
- Use copied() instead of cloned() for u8 dereference
- Simplify main() signature by removing unused ExitCode return

### Miscellaneous Tasks

- Add CI workflow and update release workflow
- Update ci to reusable workflow
- Update release workflow
- Fix release workflow
- Update Lock file

### Other

- Upgrade to Rust edition 2024 and update dependencies

### Refactor

- Use Option<&str> instead of &Option<String> in get_pool and generate_secure_password
- Pre-build character pools once in main() instead of per-password
## [0.2.0] - 2026-01-11

### Documentation

- Update Readme #10, update app version to 0.2.0

### Features

- Add option to avoid ambiguous characters
- Implement smart positional arguments
- Implement pwgen-style default grid behavior
- Adjust default quantity based on column flag
- Add --no-capitalize (-A) flag for legacy support
- Add --remove-chars (-r) flag to exclude specific characters #14

### Miscellaneous Tasks

- Update actions #24

### Other

- Update lockfile for new targets

### Refactor

- Decouple password and PIN generation logic to separate fn
- Update short and long flags for ambiguity to match with pwgen
- Unify password and pin generation logic #20

### Testing

- Add unit tests for pool logic and bump dependencies
- Add integration tests #17
## [0.1.0] - 2024-04-13
