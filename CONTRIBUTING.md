# Contributing to Cliflare

## Please do

* Fork
* Branch
* Do the things
* Commit
* Push
* Pull request

This is my first project as a Rustacean.
_**please**_ let me know if I'm missing the mark on anything.
My mom tells me that I'm very easy to work with.

Let's stick with the [Rust Code of Conduct](https://www.rust-lang.org/conduct.html).

## Changelog

This project follows semantic versioning.

Possible log types:

* `[Added]` for new features.
* `[Changed]` for changes in existing functionality.
* `[Deprecated]` for once-stable features removed in upcoming releases.
* `[Removed]` for deprecated features removed in this release.
* `[Fixed]` for any bug fixes.
* `[Security]` to invite users to upgrade in case of vulnerabilities.

Lint markdown locally:

```bash
docker run --rm \
    -v "$(pwd)/CHANGELOG.md:/CHANGELOG.md:ro" \
    avtodev/markdown-lint:v1 \
    --rules /lint/rules/changelog.js \
    --config /lint/config/changelog.yml \
    /CHANGELOG.md
```
