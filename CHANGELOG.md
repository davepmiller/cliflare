# Changelog

This project follows semantic versioning.

Possible log types:

- `[added]` for new features.
- `[changed]` for changes in existing functionality.
- `[deprecated]` for once-stable features removed in upcoming releases.
- `[removed]` for deprecated features removed in this release.
- `[fixed]` for any bug fixes.
- `[security]` to invite users to upgrade in case of vulnerabilities.

### V0.1.6 (2023-12-14)

- [added] added `zone dns export`
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