# Usage

## [Token Verify 📝](https://developers.cloudflare.com/api/operations/user-api-tokens-verify-token)

```bash
cliflare token verify
```

## [Zone List 📝](https://developers.cloudflare.com/api/operations/zones-get)

```bash
# print out all zone info
cliflare zone list
# print only zone "name" field -- i.e. domains
cliflare zone list --domains
```

## [Create a Zone 📝](https://developers.cloudflare.com/api/operations/zones-post)

```bash
cliflare zone create newzone.com
```

## [Delete a Zone 📝](https://developers.cloudflare.com/api/operations/zones-0-delete)

```bash
cliflare zone delete newzone.com
```

## [List DNS Records For A Zone 📝](https://developers.cloudflare.com/api/operations/dns-records-for-a-zone-list-dns-records)

```bash
cliflare dns list --zone_id <ZONE_ID>
cliflare dns list --zone_name <DOMAIN>
```

## [Export DNS Records For A Zone 📝](https://developers.cloudflare.com/api/operations/dns-records-for-a-zone-export-dns-records)

```bash
cliflare dns export --zone_id <ZONE_ID>
cliflare dns export --zone_name <DOMAIN>
```

## [Import DNS Records For A Zone 📝](https://developers.cloudflare.com/api/operations/dns-records-for-a-zone-import-dns-records)

```bash
cliflare dns import --file <PATH> --zone_id <ZONE_ID>
cliflare dns import -file <PATH> --zone_name <DOMAIN>
```

## [Delete All DNS Records For A Zone 📝](https://developers.cloudflare.com/api/operations/dns-records-for-a-zone-delete-dns-record)

```bash
cliflare dns clear --zone_id <ZONE_ID>
cliflare dns clear --zone_name <DOMAIN>
```

## [List All Settings For A Zone 📝](https://developers.cloudflare.com/api/operations/zone-settings-get-all-zone-settings)

```bash
cliflare settings list --zone_id <ZONE_ID>
cliflare settings list --zone_name <DOMAIN>
```