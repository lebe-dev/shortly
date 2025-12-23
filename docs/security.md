# Security

## Best practices

- Enable [authentication](configuration/gitlab.md)
- Use [nginx](configuration/nginx.md) with ip rate limiting in front of Shortly app.

## Image scan (CVE status) - December 23, 2025

```bash
trivy image docker.io/tinyops/shortly:1.0.0
2025-12-23T12:22:52+03:00	INFO	[vulndb] Need to update DB
2025-12-23T12:22:52+03:00	INFO	[vulndb] Downloading vulnerability DB...
2025-12-23T12:22:52+03:00	INFO	[vulndb] Downloading artifact...	repo="mirror.gcr.io/aquasec/trivy-db:2"
78.75 MiB / 78.75 MiB [-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------] 100.00% 4.19 MiB p/s 19s
2025-12-23T12:23:13+03:00	INFO	[vulndb] Artifact successfully downloaded	repo="mirror.gcr.io/aquasec/trivy-db:2"
2025-12-23T12:23:13+03:00	INFO	[vuln] Vulnerability scanning is enabled
2025-12-23T12:23:13+03:00	INFO	[secret] Secret scanning is enabled
2025-12-23T12:23:13+03:00	INFO	[secret] If your scanning is slow, please try '--scanners vuln' to disable secret scanning
2025-12-23T12:23:13+03:00	INFO	[secret] Please see https://trivy.dev/docs/v0.68/guide/scanner/secret#recommendation for faster secret detection
2025-12-23T12:23:13+03:00	INFO	Detected OS	family="alpine" version="3.23.2"
2025-12-23T12:23:13+03:00	WARN	This OS version is not on the EOL list	family="alpine" version="3.23"
2025-12-23T12:23:13+03:00	INFO	[alpine] Detecting vulnerabilities...	os_version="3.23" repository="3.23" pkg_num=16
2025-12-23T12:23:13+03:00	INFO	Number of language-specific files	num=0

Report Summary

┌─────────────────────────────────────────────────┬────────┬─────────────────┬─────────┐
│                     Target                      │  Type  │ Vulnerabilities │ Secrets │
├─────────────────────────────────────────────────┼────────┼─────────────────┼─────────┤
│ docker.io/tinyops/shortly:1.0.0 (alpine 3.23.2) │ alpine │        0        │    -    │
└─────────────────────────────────────────────────┴────────┴─────────────────┴─────────┘
Legend:
- '-': Not scanned
- '0': Clean (no security findings detected)
```
