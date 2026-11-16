use std::{
    collections::HashMap,
    net::IpAddr,
    sync::Mutex,
    time::{Duration, Instant},
};

use axum::{
    extract::{ConnectInfo, Request, State},
    http::{HeaderMap, HeaderValue, StatusCode, header},
    middleware::Next,
    response::{IntoResponse, Response},
};
use std::sync::Arc;

use crate::{SharedAppState, domain::config::model::link_access::LinkAccessConfig};

#[derive(Clone, Copy, Debug)]
pub struct ClientNetwork {
    pub ip: Option<IpAddr>,
    pub internal: bool,
}

fn normalize(ip: IpAddr) -> IpAddr {
    match ip {
        IpAddr::V6(ip) => ip
            .to_ipv4_mapped()
            .map(IpAddr::V4)
            .unwrap_or(IpAddr::V6(ip)),
        ip => ip,
    }
}

impl LinkAccessConfig {
    pub fn client_network(&self, peer: Option<IpAddr>, headers: &HeaderMap) -> ClientNetwork {
        let trusted = |ip: &IpAddr| {
            self.trusted_proxies
                .iter()
                .any(|network| network.contains(ip))
        };
        let ip = peer.map(normalize).and_then(|peer| {
            if !trusted(&peer) {
                return Some(peer);
            }
            // A trusted proxy must supply a complete, valid chain. Never fall back to its own IP.
            let mut chain = Vec::new();
            for value in headers.get_all("x-forwarded-for") {
                for value in value.to_str().ok()?.split(',') {
                    chain.push(normalize(value.trim().parse::<IpAddr>().ok()?));
                    if chain.len() > 32 {
                        return None;
                    }
                }
            }
            chain.into_iter().rev().find(|ip| !trusted(ip))
        });
        ClientNetwork {
            internal: ip.is_some_and(|ip| {
                self.internal_networks
                    .iter()
                    .any(|network| network.contains(&ip))
            }),
            ip,
        }
    }
}

/// Fixed one-minute windows, bounded to prevent unbounded growth from many source addresses.
#[derive(Default)]
pub struct NameCheckLimiter(Mutex<HashMap<Option<IpAddr>, (Instant, u32)>>);

impl NameCheckLimiter {
    pub fn check(&self, ip: Option<IpAddr>, limit: u32) -> Result<(), u64> {
        self.check_at(ip, limit, Instant::now())
    }

    fn check_at(&self, ip: Option<IpAddr>, limit: u32, now: Instant) -> Result<(), u64> {
        // Treat an IPv6 /64 as a single source, so rotating interface IDs doesn't reset the limit.
        let key = ip.map(normalize).map(|ip| match ip {
            IpAddr::V6(ip) => IpAddr::V6((u128::from(ip) & (u128::MAX << 64)).into()),
            ip => ip,
        });
        let Ok(mut entries) = self.0.lock() else {
            return Err(60);
        };
        if entries.len() >= 10_000 {
            entries.retain(|_, (start, _)| now.duration_since(*start) < Duration::from_secs(60));
            if entries.len() >= 10_000 && !entries.contains_key(&key) {
                return Err(60);
            }
        }
        let (start, count) = entries.entry(key).or_insert((now, 0));
        if now.duration_since(*start) >= Duration::from_secs(60) {
            *start = now;
            *count = 0;
        }
        if *count >= limit {
            return Err(60 - now.duration_since(*start).as_secs());
        }
        *count += 1;
        Ok(())
    }
}

pub fn too_many_checks(retry_after: u64) -> Response {
    (
        StatusCode::TOO_MANY_REQUESTS,
        [(header::RETRY_AFTER, retry_after.to_string())],
        axum::Json(serde_json::json!({
            "code": "name_check_rate_limited",
            "error": "Too many name checks; try again later",
        })),
    )
        .into_response()
}

pub async fn network_middleware(
    State(state): State<Arc<SharedAppState>>,
    mut request: Request,
    next: Next,
) -> Response {
    let peer = request
        .extensions()
        .get::<ConnectInfo<std::net::SocketAddr>>()
        .map(|info| info.0.ip());
    let client = state.link_access.client_network(peer, request.headers());
    request.extensions_mut().insert(client);
    let path = request.uri().path();
    // Keep the redirect classification in sync with static_handler, which accepts
    // multiple leading slashes when resolving a link.
    let link_path = path.trim_start_matches('/');
    let no_store = path == "/api/config"
        || path == "/api/user/urls"
        || path.starts_with("/api/url")
        || (!link_path.is_empty() && !link_path.contains('/') && !link_path.contains('.'));
    let mut response = next.run(request).await;
    if no_store {
        response.headers_mut().insert(
            header::CACHE_CONTROL,
            HeaderValue::from_static("private, no-store"),
        );
    }
    response
}

#[cfg(test)]
mod tests {
    use super::*;
    use server_lib::domain::url::model::UrlAccess;

    fn config() -> LinkAccessConfig {
        LinkAccessConfig {
            default_access: UrlAccess::Public,
            allow_override: false,
            internal_networks: vec![
                "10.20.0.0/16".parse().unwrap(),
                "fd12:3456::/32".parse().unwrap(),
            ],
            trusted_proxies: vec![
                "127.0.0.1/32".parse().unwrap(),
                "172.18.0.0/16".parse().unwrap(),
            ],
            checks_per_minute: 30,
        }
    }

    fn client(peer: Option<&str>, forwarded: Option<&str>) -> ClientNetwork {
        let mut headers = HeaderMap::new();
        if let Some(value) = forwarded {
            headers.insert("x-forwarded-for", value.parse().unwrap());
        }
        config().client_network(peer.map(|ip| ip.parse().unwrap()), &headers)
    }

    #[test]
    fn direct_clients_and_untrusted_headers() {
        assert!(client(Some("10.20.0.1"), None).internal);
        assert!(client(Some("10.20.255.255"), None).internal);
        assert!(!client(Some("10.21.0.0"), None).internal);
        assert!(!client(Some("198.51.100.1"), Some("10.20.0.1")).internal);
        assert!(!client(None, Some("10.20.0.1")).internal);
        assert!(client(Some("fd12:3456::1"), None).internal);
        assert!(!client(Some("fd12:3457::1"), None).internal);
        assert!(client(Some("::ffff:10.20.0.1"), None).internal);
    }

    #[test]
    fn trusted_proxy_chain_ignores_spoofed_prefix() {
        assert!(client(Some("127.0.0.1"), Some("10.20.0.1")).internal);
        assert!(client(Some("127.0.0.1"), Some("10.20.0.1, 172.18.0.5")).internal);
        let outside = client(
            Some("127.0.0.1"),
            Some("10.20.0.1, 198.51.100.9, 172.18.0.5"),
        );
        assert!(!outside.internal);
        assert_eq!(outside.ip, Some("198.51.100.9".parse().unwrap()));
        assert!(client(Some("127.0.0.1"), Some("fd12:3456::1, 172.18.0.5")).internal);
    }

    #[test]
    fn invalid_or_missing_forwarding_never_uses_proxy_address() {
        for value in [
            None,
            Some(""),
            Some("10.20.0.1, garbage"),
            Some("10.20.0.1,"),
            Some("127.0.0.1,172.18.0.5"),
        ] {
            let result = client(Some("127.0.0.1"), value);
            assert_eq!(result.ip, None, "{value:?}");
            assert!(!result.internal);
        }
        assert!(!client(Some("127.0.0.1"), Some(&vec!["10.20.0.1"; 33].join(","))).internal);
        let mut headers = HeaderMap::new();
        headers.append("x-forwarded-for", "10.20.0.1".parse().unwrap());
        headers.append("x-forwarded-for", "198.51.100.9".parse().unwrap());
        assert!(
            !config()
                .client_network(Some("127.0.0.1".parse().unwrap()), &headers)
                .internal
        );
        headers.insert("x-forwarded-for", HeaderValue::from_bytes(&[0xff]).unwrap());
        assert!(
            !config()
                .client_network(Some("127.0.0.1".parse().unwrap()), &headers)
                .internal
        );
    }

    #[test]
    fn limiter_enforces_windows_and_isolates_clients() {
        let limiter = NameCheckLimiter::default();
        let now = Instant::now();
        let ip = Some("192.0.2.1".parse().unwrap());
        assert!(limiter.check_at(ip, 2, now).is_ok());
        assert!(limiter.check_at(ip, 2, now).is_ok());
        assert_eq!(limiter.check_at(ip, 2, now), Err(60));
        assert_eq!(
            limiter.check_at(ip, 2, now + Duration::from_secs(59)),
            Err(1)
        );
        assert!(
            limiter
                .check_at(Some("192.0.2.2".parse().unwrap()), 2, now)
                .is_ok()
        );
        assert!(
            limiter
                .check_at(ip, 2, now + Duration::from_secs(60))
                .is_ok()
        );
        assert!(limiter.check_at(None, 1, now).is_ok());
        assert!(limiter.check_at(None, 1, now).is_err());
    }

    #[test]
    fn limiter_groups_ipv6_and_bounds_memory() {
        let limiter = NameCheckLimiter::default();
        let now = Instant::now();
        assert!(
            limiter
                .check_at(Some("2001:db8::1".parse().unwrap()), 1, now)
                .is_ok()
        );
        assert!(
            limiter
                .check_at(Some("2001:db8::2".parse().unwrap()), 1, now)
                .is_err()
        );
        assert!(
            limiter
                .check_at(Some("2001:db8:0:1::1".parse().unwrap()), 1, now)
                .is_ok()
        );
        for i in 0..10_000 {
            let _ = limiter.check_at(Some(IpAddr::V4(i.into())), 1, now);
        }
        assert_eq!(limiter.0.lock().unwrap().len(), 10_000);
        assert!(
            limiter
                .check_at(Some("203.0.113.8".parse().unwrap()), 1, now)
                .is_err()
        );
        assert!(
            limiter
                .check_at(
                    Some("203.0.113.8".parse().unwrap()),
                    1,
                    now + Duration::from_secs(60)
                )
                .is_ok()
        );
        assert_eq!(limiter.0.lock().unwrap().len(), 1);
    }
}
