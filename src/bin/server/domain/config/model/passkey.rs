use std::fmt;
use thiserror::Error;

/// Default lifetime of an unfinished WebAuthn ceremony, in seconds.
const DEFAULT_CHALLENGE_TTL: i64 = 300;

/// Default name shown by the authenticator during registration.
const DEFAULT_RP_NAME: &str = "Shortly";

/// Passkey settings. Unlike the rest of the application configuration these
/// values come from the environment only (a `.env` file is loaded at startup),
/// so `config.yml` carries nothing about passkeys.
#[derive(PartialEq, Clone, Debug)]
pub struct PasskeyConfig {
    /// Relying party ID: the registrable domain of the service, without scheme or port
    pub rp_id: String,
    /// Origin the browser must report, including scheme and port
    pub rp_origin: String,
    /// Name shown by the authenticator
    pub rp_name: String,
    /// Lifetime of an unfinished ceremony, in seconds
    pub challenge_ttl: i64,
}

#[derive(Debug, Error)]
pub enum PasskeyConfigError {
    #[error("{0} must be set when PASSKEY_ENABLED is true")]
    MissingValue(&'static str),

    #[error("{0} must be a positive number of seconds")]
    InvalidChallengeTtl(&'static str),
}

impl PasskeyConfig {
    /// Read the passkey settings from the environment.
    ///
    /// Returns `None` when `PASSKEY_ENABLED` is not set to `true`, and an error
    /// when the feature is switched on but a required value is missing.
    pub fn from_env() -> Result<Option<Self>, PasskeyConfigError> {
        if !read_flag("PASSKEY_ENABLED") {
            return Ok(None);
        }

        let rp_id =
            read_value("PASSKEY_RP_ID").ok_or(PasskeyConfigError::MissingValue("PASSKEY_RP_ID"))?;

        let rp_origin = read_value("PASSKEY_RP_ORIGIN")
            .ok_or(PasskeyConfigError::MissingValue("PASSKEY_RP_ORIGIN"))?;

        let rp_name = read_value("PASSKEY_RP_NAME").unwrap_or_else(|| DEFAULT_RP_NAME.to_string());

        let challenge_ttl = match read_value("PASSKEY_CHALLENGE_TTL") {
            Some(raw) => raw.parse::<i64>().ok().filter(|value| *value > 0).ok_or(
                PasskeyConfigError::InvalidChallengeTtl("PASSKEY_CHALLENGE_TTL"),
            )?,
            None => DEFAULT_CHALLENGE_TTL,
        };

        Ok(Some(PasskeyConfig {
            rp_id,
            rp_origin,
            rp_name,
            challenge_ttl,
        }))
    }
}

fn read_value(name: &str) -> Option<String> {
    std::env::var(name)
        .ok()
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty())
}

fn read_flag(name: &str) -> bool {
    read_value(name)
        .map(|value| value.eq_ignore_ascii_case("true") || value == "1")
        .unwrap_or(false)
}

impl fmt::Display for PasskeyConfig {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "PasskeyConfig {{ rp_id: {}, rp_origin: {}, rp_name: {}, challenge_ttl: {} }}",
            self.rp_id, self.rp_origin, self.rp_name, self.challenge_ttl
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serial_test::serial;

    fn clear_env() {
        unsafe {
            std::env::remove_var("PASSKEY_ENABLED");
            std::env::remove_var("PASSKEY_RP_ID");
            std::env::remove_var("PASSKEY_RP_ORIGIN");
            std::env::remove_var("PASSKEY_RP_NAME");
            std::env::remove_var("PASSKEY_CHALLENGE_TTL");
        }
    }

    #[test]
    #[serial]
    fn test_disabled_by_default() {
        clear_env();

        assert!(PasskeyConfig::from_env().unwrap().is_none());
    }

    #[test]
    #[serial]
    fn test_disabled_when_flag_is_false() {
        clear_env();
        unsafe {
            std::env::set_var("PASSKEY_ENABLED", "false");
            std::env::set_var("PASSKEY_RP_ID", "shortly.company.com");
            std::env::set_var("PASSKEY_RP_ORIGIN", "https://shortly.company.com");
        }

        assert!(PasskeyConfig::from_env().unwrap().is_none());

        clear_env();
    }

    #[test]
    #[serial]
    fn test_defaults_are_applied() {
        clear_env();
        unsafe {
            std::env::set_var("PASSKEY_ENABLED", "true");
            std::env::set_var("PASSKEY_RP_ID", "shortly.company.com");
            std::env::set_var("PASSKEY_RP_ORIGIN", "https://shortly.company.com");
        }

        let config = PasskeyConfig::from_env().unwrap().unwrap();

        assert_eq!(config.rp_id, "shortly.company.com");
        assert_eq!(config.rp_origin, "https://shortly.company.com");
        assert_eq!(config.rp_name, DEFAULT_RP_NAME);
        assert_eq!(config.challenge_ttl, DEFAULT_CHALLENGE_TTL);

        clear_env();
    }

    #[test]
    #[serial]
    fn test_custom_values() {
        clear_env();
        unsafe {
            std::env::set_var("PASSKEY_ENABLED", "1");
            std::env::set_var("PASSKEY_RP_ID", "localhost");
            std::env::set_var("PASSKEY_RP_ORIGIN", "http://localhost:8080");
            std::env::set_var("PASSKEY_RP_NAME", "Shortly Dev");
            std::env::set_var("PASSKEY_CHALLENGE_TTL", "60");
        }

        let config = PasskeyConfig::from_env().unwrap().unwrap();

        assert_eq!(config.rp_id, "localhost");
        assert_eq!(config.rp_origin, "http://localhost:8080");
        assert_eq!(config.rp_name, "Shortly Dev");
        assert_eq!(config.challenge_ttl, 60);

        clear_env();
    }

    #[test]
    #[serial]
    fn test_missing_rp_id_is_rejected() {
        clear_env();
        unsafe {
            std::env::set_var("PASSKEY_ENABLED", "true");
            std::env::set_var("PASSKEY_RP_ORIGIN", "https://shortly.company.com");
        }

        let result = PasskeyConfig::from_env();

        assert!(matches!(
            result,
            Err(PasskeyConfigError::MissingValue("PASSKEY_RP_ID"))
        ));

        clear_env();
    }

    #[test]
    #[serial]
    fn test_missing_rp_origin_is_rejected() {
        clear_env();
        unsafe {
            std::env::set_var("PASSKEY_ENABLED", "true");
            std::env::set_var("PASSKEY_RP_ID", "shortly.company.com");
        }

        let result = PasskeyConfig::from_env();

        assert!(matches!(
            result,
            Err(PasskeyConfigError::MissingValue("PASSKEY_RP_ORIGIN"))
        ));

        clear_env();
    }

    #[test]
    #[serial]
    fn test_invalid_challenge_ttl_is_rejected() {
        clear_env();
        unsafe {
            std::env::set_var("PASSKEY_ENABLED", "true");
            std::env::set_var("PASSKEY_RP_ID", "shortly.company.com");
            std::env::set_var("PASSKEY_RP_ORIGIN", "https://shortly.company.com");
            std::env::set_var("PASSKEY_CHALLENGE_TTL", "0");
        }

        let result = PasskeyConfig::from_env();

        assert!(matches!(
            result,
            Err(PasskeyConfigError::InvalidChallengeTtl(_))
        ));

        clear_env();
    }
}
