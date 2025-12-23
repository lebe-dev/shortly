# Integration with HashiCorp Vault

```bash
vault policy write shortly - <<EOF
path "secret_v2/data/infra/shortly" {
  capabilities = ["read"]
}
EOF

vault write auth/kubernetes/role/shortly policies=shortly bound_service_account_names=shortly,vault,vault-secrets-webhook bound_service_account_namespaces=shortly,vault ttl=24h
```
