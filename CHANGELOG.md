# Changelog

## v0.4.3 - 2024-01-07

- [Added] `sanity check coverage, happy path unit tests with API mocks`

## v0.4.2 - 2024-01-07

- [Added] `nothing really`

## v0.4.1 - 2024-01-07

- [Changed] made cloudflare client endpoint an environment variable
- [Added] `settings list --name <DOMAIN>`
- [Changed] test setup for rest client mocks

## v0.4.0 - 2023-12-18

- [Fixed] badge links in README
- [Added] `dns clear --name <DOMAIN>`
- [Changed] moved environment variable management into tests, while testing
- [Added] some cool spice to the README (cliffy.jpeg)

## v0.3.4 - 2023-12-16

- [Added] this version got lost due to CI updates

## v0.3.3 - 2023-12-16

- [Added] markdown linting to ci
- [Changed] publish action depends on ci - reduce redundancy
- [Fixed] lints in *.md files
- [Changed] clippy ci to use `clippy::pedantic`
- [Fixed] pedantic lints

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
