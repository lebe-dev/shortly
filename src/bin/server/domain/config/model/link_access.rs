use ipnet::IpNet;
use server_lib::domain::url::model::UrlAccess;

#[derive(Clone, Debug)]
pub struct LinkAccessConfig {
    pub default_access: UrlAccess,
    pub allow_override: bool,
    pub internal_networks: Vec<IpNet>,
    pub trusted_proxies: Vec<IpNet>,
    pub checks_per_minute: u32,
}

impl LinkAccessConfig {
    pub fn from_env() -> anyhow::Result<Self> {
        Self::from_values(|name| std::env::var(name).ok())
    }

    fn from_values(read: impl Fn(&str) -> Option<String>) -> anyhow::Result<Self> {
        let default_access = match read("NAMED_URL_DEFAULT_ACCESS")
            .as_deref()
            .unwrap_or("public")
        {
            "public" => UrlAccess::Public,
            "internal" => UrlAccess::Internal,
            _ => anyhow::bail!("NAMED_URL_DEFAULT_ACCESS must be public or internal"),
        };
        let allow_override = match read("NAMED_URL_ALLOW_ACCESS_OVERRIDE")
            .as_deref()
            .unwrap_or("false")
        {
            "true" | "1" => true,
            "false" | "0" => false,
            _ => anyhow::bail!("NAMED_URL_ALLOW_ACCESS_OVERRIDE must be true or false"),
        };
        let networks = |name| -> anyhow::Result<Vec<IpNet>> {
            let raw = read(name).unwrap_or_default();
            if raw.trim().is_empty() {
                return Ok(Vec::new());
            }
            raw.split(',')
                .map(|cidr| {
                    cidr.trim()
                        .parse()
                        .map_err(|_| anyhow::anyhow!("Invalid CIDR in {name}: {cidr}"))
                })
                .collect()
        };
        let internal_networks = networks("INTERNAL_NETWORK_CIDRS")?;
        let trusted_proxies = networks("TRUSTED_PROXY_CIDRS")?;
        anyhow::ensure!(
            !(default_access == UrlAccess::Internal || allow_override)
                || !internal_networks.is_empty(),
            "INTERNAL_NETWORK_CIDRS must be set when internal link creation is enabled"
        );
        let checks_per_minute = read("NAMED_URL_CHECK_RATE_LIMIT")
            .unwrap_or_else(|| "30".into())
            .parse::<u32>()
            .ok()
            .filter(|limit| *limit > 0)
            .ok_or_else(|| {
                anyhow::anyhow!("NAMED_URL_CHECK_RATE_LIMIT must be a positive integer")
            })?;
        Ok(Self {
            default_access,
            allow_override,
            internal_networks,
            trusted_proxies,
            checks_per_minute,
        })
    }

    pub fn creation_access(
        &self,
        named: bool,
        requested: Option<UrlAccess>,
    ) -> Result<UrlAccess, &'static str> {
        if !named {
            return if requested.is_some() {
                Err("Access mode is only supported for named URLs")
            } else {
                Ok(UrlAccess::Public)
            };
        }
        if !self.allow_override && requested.is_some_and(|access| access != self.default_access) {
            return Err("Access mode override is disabled");
        }
        Ok(requested.unwrap_or(self.default_access))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn config(values: &[(&str, &str)]) -> anyhow::Result<LinkAccessConfig> {
        LinkAccessConfig::from_values(|key| {
            values
                .iter()
                .find(|(name, _)| *name == key)
                .map(|(_, value)| value.to_string())
        })
    }

    #[test]
    fn defaults_and_creation_policy() {
        let settings = config(&[]).unwrap();
        assert_eq!(settings.default_access, UrlAccess::Public);
        assert!(!settings.allow_override);
        assert!(settings.internal_networks.is_empty());
        assert!(settings.trusted_proxies.is_empty());
        assert_eq!(settings.checks_per_minute, 30);
        assert!(
            settings
                .creation_access(true, Some(UrlAccess::Internal))
                .is_err()
        );
        assert!(
            settings
                .creation_access(false, Some(UrlAccess::Public))
                .is_err()
        );
        assert_eq!(
            settings.creation_access(true, None).unwrap(),
            UrlAccess::Public
        );
    }

    #[test]
    fn internal_default_and_optional_override() {
        let mut settings = config(&[
            ("NAMED_URL_DEFAULT_ACCESS", "internal"),
            ("INTERNAL_NETWORK_CIDRS", "10.2.0.0/16,fd12::/32"),
        ])
        .unwrap();
        assert_eq!(
            settings.creation_access(true, None).unwrap(),
            UrlAccess::Internal
        );
        assert_eq!(
            settings.creation_access(false, None).unwrap(),
            UrlAccess::Public
        );
        assert!(
            settings
                .creation_access(true, Some(UrlAccess::Public))
                .is_err()
        );
        settings.allow_override = true;
        assert_eq!(
            settings
                .creation_access(true, Some(UrlAccess::Public))
                .unwrap(),
            UrlAccess::Public
        );
        settings.default_access = UrlAccess::Public;
        assert_eq!(
            settings
                .creation_access(true, Some(UrlAccess::Internal))
                .unwrap(),
            UrlAccess::Internal
        );
    }

    #[test]
    fn invalid_configuration_is_rejected() {
        for values in [
            vec![("NAMED_URL_DEFAULT_ACCESS", "corporate")],
            vec![("NAMED_URL_DEFAULT_ACCESS", "internal")],
            vec![("NAMED_URL_ALLOW_ACCESS_OVERRIDE", "true")],
            vec![("NAMED_URL_ALLOW_ACCESS_OVERRIDE", "typo")],
            vec![("INTERNAL_NETWORK_CIDRS", "10.0.0.0/33")],
            vec![("TRUSTED_PROXY_CIDRS", "0.0.0.0/0,")],
            vec![("NAMED_URL_CHECK_RATE_LIMIT", "0")],
        ] {
            assert!(config(&values).is_err(), "{values:?}");
        }
    }
}
