# cloudflare
CLI tool to interact with Cloudflare APIs. An excuse to write some Rust.

Current functionality:
* [Cloudflare API - Verify Token](https://developers.cloudflare.com/api/operations/user-api-tokens-verify-token)
```bash
CLOUDFLARE_TOKEN="$CLOUDFLARE_TOKEN"; cargo run -- token verify
```
* [Zone List](https://developers.cloudflare.com/api/operations/zones-get)
```bash
# print out all zone info
CLOUDFLARE_TOKEN="$CLOUDFLARE_TOKEN"; cargo run -- zone list
# print domain name for each zone
cargo run -- zone list --domains
```

Coming soon:
* [Zone Details](https://developers.cloudflare.com/api/operations/zones-0-get)
* [Create Zone](https://developers.cloudflare.com/api/operations/zones-post)
* [List DNS Records](https://developers.cloudflare.com/api/operations/dns-records-for-a-zone-list-dns-records)
* [Create DNS Record](https://developers.cloudflare.com/api/operations/dns-records-for-a-zone-create-dns-record)

Coming after:
* Proper crate and homebrew setup
* Create WAF rules
* Create redirect rules
* Create some other rules
