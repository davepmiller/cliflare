# ![cliffy](assets/cliffy.jpeg)

[![build](https://github.com/davepmiller/cliflare/actions/workflows/ci.yml/badge.svg?branch=main)](<https://github.com/davepmiller/cliflare/actions/workflows/ci.yml>)
[![publish](https://github.com/davepmiller/cliflare/actions/workflows/publish.yml/badge.svg?branch=main)](<https://github.com/davepmiller/cliflare/actions/workflows/publish.yml>)
[![Coverage Status](https://coveralls.io/repos/github/davepmiller/cliflare/badge.svg)](https://coveralls.io/github/davepmiller/cliflare)
[![crates.io](https://img.shields.io/crates/v/cliflare.svg)](<https://crates.io/crates/cliflare>)
[![downloads](https://img.shields.io/crates/d/cliflare)](<https://crates.io/crates/cliflare>)
[![license](https://img.shields.io/badge/license-MIT-green.svg)](<https://opensource.org/licenses/MIT>)
[![license](https://img.shields.io/badge/license-APACHE-blue.svg)](<https://opensource.org/licenses/APACHE>)

* 🛠 CLI️ to interact with Cloudflare APIs
* 🥳 An excuse to write some Rust
* 👷 Under heavy development

## Setup

### [Install Rust 📝](https://www.rust-lang.org/tools/install)

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### Install Cliflare

```bash
cargo install cliflare
```

### [Generate a Cloudflare API token 📝](https://developers.cloudflare.com/cloudflare-one/api-terraform/scoped-api-tokens/)

### [Grab Account And Zone IDs 📝](https://developers.cloudflare.com/fundamentals/setup/find-account-and-zone-ids/)

### Environment

```bash
# add your token value to a startup script
echo CLOUDFLARE_ENDPOINT=https://api.cloudflare.com/client/v4 >> ~/.zshrc
echo CLOUDFLARE_TOKEN=abcd1234**API_TOKEN**4321dcba >> ~/.zshrc
echo CLOUDFLARE_ACCOUNT_ID=abcd1234**ACCOUNT_ID**4321dcba >> ~/.zshrc
```

## Functionality

See examples in [usage.md](docs/usage.md)

* token verify
* zone [ list | create | delete ]
* dns [ list | import | export | delete all ]
* settings list

## License

Cliflare is free software.
This project is available under the terms of
either the Apache 2.0 license or the MIT license.
Cliflare is provided on an "AS IS" basis and there is NO WARRANTY attached to it.

## Feature Requests

Fill out [this form](https://docs.google.com/forms/d/e/1FAIpQLSfDBhmvtRn1C3Vzi_nplHV9QyBVbPUfdqhziUj_sWYyi-XIFw/viewform?usp=sf_link)
and I'll get back to you.
