# ☁️🚀 cliflare 🚀☁️
* CLI 🛠️ to interact with Cloudflare APIs.
* An excuse to write some Rust 🥳

#### Setup:
* [Install Rust 📝](https://www.rust-lang.org/tools/install)
    ```bash
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
    ```
* Clone
    ```bash
    git clone git@github.com:davepmiller/cliflare.git
    ```
* Build
  ```bash
  cargo build --release
  ```
* [Generate a Cloudflare API token 📝](https://developers.cloudflare.com/cloudflare-one/api-terraform/scoped-api-tokens/)
* [Grab Account And Zone IDs 📝](https://developers.cloudflare.com/fundamentals/setup/find-account-and-zone-ids/)
* Environment
  ```bash
  # add your token value to a startup script
  echo CLOUDFLARE_TOKEN=abcd1234**API_TOKEN**4321dcba >> ~/.zshrc
  echo CLOUDFLARE_ACCOUNT_ID=abcd1234**ACCOUNT_ID**4321dcba >> ~/.zshrc
  ```
#### Execute:
* [Token Verify 📝](https://developers.cloudflare.com/api/operations/user-api-tokens-verify-token)
    ```bash
    cargo run -- token verify
    ```
* [Zone List 📝](https://developers.cloudflare.com/api/operations/zones-get)
    ```bash
    # print out all zone info
    cargo run -- zone list
    # print domain name for each zone
    cargo run -- zone list --domains
    ```
* [Create Zone 📝](https://developers.cloudflare.com/api/operations/zones-post)
  ```bash
  cargo run -- zone create newzone.com
  ```

Coming soon:
* [Zone Details](https://developers.cloudflare.com/api/operations/zones-0-get)
* [List DNS Records For A Zone](https://developers.cloudflare.com/api/operations/dns-records-for-a-zone-list-dns-records)
* [Create DNS Record For A Zone](https://developers.cloudflare.com/api/operations/dns-records-for-a-zone-create-dns-record)

Coming after:
* Proper crate and homebrew setup
* Create WAF rules
* Create redirect rules
* Create some other rules
