# Security

## Best practices

- Use [nginx](nginx.md) with ip rate limiting in front of Shortly app.

## Image scan (CVE status) - November 18, 2025

```bash
2025-11-18T16:49:32+03:00	INFO	[vulndb] Need to update DB
2025-11-18T16:49:32+03:00	INFO	[vulndb] Downloading vulnerability DB...
2025-11-18T16:49:32+03:00	INFO	[vulndb] Downloading artifact...	repo="mirror.gcr.io/aquasec/trivy-db:2"
75.19 MiB / 75.19 MiB [------------------------------------------------------------------------------------------------------------------------] 100.00% 3.86 MiB p/s 20s
2025-11-18T16:49:53+03:00	INFO	[vulndb] Artifact successfully downloaded	repo="mirror.gcr.io/aquasec/trivy-db:2"
2025-11-18T16:49:53+03:00	INFO	[vuln] Vulnerability scanning is enabled
2025-11-18T16:49:53+03:00	INFO	[secret] Secret scanning is enabled
2025-11-18T16:49:53+03:00	INFO	[secret] If your scanning is slow, please try '--scanners vuln' to disable secret scanning
2025-11-18T16:49:53+03:00	INFO	[secret] Please see https://trivy.dev/v0.67/docs/scanner/secret#recommendation for faster secret detection
2025-11-18T16:49:54+03:00	INFO	Detected OS	family="alpine" version="3.22.2"
2025-11-18T16:49:54+03:00	INFO	[alpine] Detecting vulnerabilities...	os_version="3.22" repository="3.22" pkg_num=21
2025-11-18T16:49:54+03:00	INFO	Number of language-specific files	num=0

Report Summary

┌─────────────────────────┬────────┬─────────────────┬─────────┐
│         Target          │  Type  │ Vulnerabilities │ Secrets │
├─────────────────────────┼────────┼─────────────────┼─────────┤
│ app:dev (alpine 3.22.2) │ alpine │        0        │    -    │
└─────────────────────────┴────────┴─────────────────┴─────────┘
Legend:
- '-': Not scanned
- '0': Clean (no security findings detected)
```
