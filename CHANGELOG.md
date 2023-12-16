# Changelog

## v0.3.2 - 2023-12-15

- [Added] `zone delete --name <DOMAIN>`
- [Added] `dns import --file <PATH> --name <DOMAIN>`
- [Added] link for feature requests

## v0.3.1 - 2023-12-15

- [Fixed] `zone dns export` output
  The output needed to be usable by `zone dns import`

## v0.3.0 - 2023-12-15

- [Fixed] fixed Cargo.toml -> package -> repository
- [Changed] migrated CI rust toolchain -> `dtolnay/rust-toolchain`
  This action is current, no more deprecation warnings!
- [Changed] updated CI git checkout action `actions/checkout@v4`

## v0.2.0 - 2023-12-14

- [Added] `zone dns export`
- [Changed] Remove tokio async, use reqwest::blocking
  This was the only way I could figure out mocking for tests.
- [Added] Added `CODE_OF_CONDUCT.md`
- [Added] Added `CONTRIBUTING.md`

## v0.1.5 - 2023-12-13

- [Changed] Changed parameter names for `zones dns list` to be more explicit. `id` to `zone_id`, `name` to `zone_name`
- [Added] Added short parameters for `zones dns list`. `z` and `i`
- [Added] Added `CHANGELOG.md`
- [Fixed] Fixed bug where `zones` get requests were returning the default `20` for `per_page`
  Bumped this to a hard-coded `per_page=100`, for now. Added todo to `README` to parameterize.
- [Changed] Moved `dns` out to top level, `zone dns` becomes just `dns`
- [Removed] Removed inconsistent types from `response.rs` in preference of `serde_json::Value`
