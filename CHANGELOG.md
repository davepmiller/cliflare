# Changelog

### V0.3.2 (2023-12-15)

- [added] `zone delete --name <DOMAIN>`
- [added] `dns import --file <PATH> --name <DOMAIN>`

### V0.3.1 (2023-12-15)

- [fixed] `zone dns export` output
  The output needed to be usable by `zone dns import`

### V0.3.0 (2023-12-15)

- [fixed] fixed Cargo.toml -> package -> repository
- [changed] migrated CI rust toolchain -> `dtolnay/rust-toolchain`
  This action is current, no more deprecation warnings!
- [changed] updated CI git checkout action `actions/checkout@v4`

### V0.2.0 (2023-12-14)

- [added] `zone dns export`
- [changed] Remove tokio async, use reqwest::blocking
  This was the only way I could figure out mocking for tests.
- [added] Added `CODE_OF_CONDUCT.md`
- [added] Added `CONTRIBUTING.md`

### V0.1.5 (2023-12-13)

- [changed] Changed parameter names for `zones dns list` to be more explicit. `id` to `zone_id`, `name` to `zone_name`.
- [added] Added short parameters for `zones dns list`. `z` and `i`
- [added] Added `CHANGELOG.md`
- [fixed] Fixed bug where `zones` get requests were returning the default `20` for `per_page`. 
  Bumped this to a hard-coded `per_page=100`, for now. Added todo to `README` to parameterize.
- [changed] Moved `dns` out to top level. `zone dns` becomes just `dns`.
- [removed] Removed inconsistent types from `response.rs` in preference of `serde_json::Value`.
