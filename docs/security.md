# Security

## Best practices

- Enable [authentication](configuration/gitlab.md)
- Use [nginx](configuration/nginx.md) with ip rate limiting in front of Shortly app.

Startup errors omit database connection strings and configuration values. OAuth errors omit provider response bodies and credentials. Application logs identify shortened links by their short IDs.

The bundled nginx access log omits query strings and Referer headers. It records callback status codes and suppresses callback-specific nginx errors that may contain the full request URI. Review access and error logging in external proxies and ingress controllers before forwarding their logs to an external service.

## Image scan (CVE status)

```bash
$ date
Thu May  7 16:31:46 MSK 2026

$ trivy image --severity HIGH,CRITICAL tinyops/shortly:1.5.0

2026-05-07T16:31:17+03:00	INFO	[vuln] Vulnerability scanning is enabled
2026-05-07T16:31:17+03:00	INFO	[secret] Secret scanning is enabled
2026-05-07T16:31:17+03:00	INFO	[secret] If your scanning is slow, please try '--scanners vuln' to disable secret scanning
2026-05-07T16:31:17+03:00	INFO	[secret] Please see https://trivy.dev/docs/v0.69/guide/scanner/secret#recommendation for faster secret detection
2026-05-07T16:31:17+03:00	INFO	Detected OS	family="alpine" version="3.23.4"
2026-05-07T16:31:17+03:00	INFO	[alpine] Detecting vulnerabilities...	os_version="3.23" repository="3.23" pkg_num=16
2026-05-07T16:31:17+03:00	INFO	Number of language-specific files	num=0

Report Summary

┌───────────────────────────────────────┬────────┬─────────────────┬─────────┐
│                Target                 │  Type  │ Vulnerabilities │ Secrets │
├───────────────────────────────────────┼────────┼─────────────────┼─────────┤
│ tinyops/shortly:1.5.0 (alpine 3.23.4) │ alpine │        0        │    -    │
└───────────────────────────────────────┴────────┴─────────────────┴─────────┘
Legend:
- '-': Not scanned
- '0': Clean (no security findings detected)
```
