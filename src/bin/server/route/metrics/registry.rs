use lazy_static::lazy_static;
use prometheus::{Gauge, GaugeVec, Histogram, HistogramOpts, Opts, Registry};

lazy_static! {
    pub static ref REGISTRY: Registry = Registry::new();

    // URL Metrics
    pub static ref URLS_TOTAL: Gauge =
        Gauge::with_opts(Opts::new("shortly_urls_total", "Total number of active URLs"))
        .expect("metric creation");

    pub static ref URLS_LAST_CREATED: Gauge =
        Gauge::with_opts(Opts::new("shortly_urls_last_created_timestamp",
            "Unix timestamp of last created URL"))
        .expect("metric creation");

    pub static ref URLS_CUSTOM_NAMED: Gauge =
        Gauge::with_opts(Opts::new("shortly_urls_custom_named_total",
            "Number of URLs with custom names"))
        .expect("metric creation");

    pub static ref URLS_EXPIRED: Gauge =
        Gauge::with_opts(Opts::new("shortly_urls_expired_total",
            "Number of expired URLs"))
        .expect("metric creation");

    pub static ref URLS_LAST_ACCESSED: Gauge =
        Gauge::with_opts(Opts::new("shortly_urls_last_accessed_timestamp",
            "Unix timestamp of the most recent URL access (redirect)"))
        .expect("metric creation");

    pub static ref URLS_DELETED_24H: Gauge =
        Gauge::with_opts(Opts::new("shortly_urls_deleted_last_24h",
            "URLs deleted in last 24 hours"))
        .expect("metric creation");

    // User Metrics
    pub static ref USERS_TOTAL: Gauge =
        Gauge::with_opts(Opts::new("shortly_users_total", "Total registered users"))
        .expect("metric creation");

    pub static ref USERS_ACTIVE_SESSIONS: Gauge =
        Gauge::with_opts(Opts::new("shortly_users_active_sessions",
            "Number of active user sessions"))
        .expect("metric creation");

    pub static ref USERS_LAST_LOGIN: Gauge =
        Gauge::with_opts(Opts::new("shortly_users_last_login_timestamp",
            "Unix timestamp of last user login"))
        .expect("metric creation");

    // Audit Metrics
    pub static ref AUDIT_EVENTS_TOTAL: GaugeVec =
        GaugeVec::new(
            Opts::new("shortly_audit_events_total", "Total audit events by type"),
            &["event_type"]
        ).expect("metric creation");

    pub static ref AUDIT_LAST_EVENT: GaugeVec =
        GaugeVec::new(
            Opts::new("shortly_audit_last_event_timestamp",
                "Last audit event timestamp by type"),
            &["event_type"]
        ).expect("metric creation");

    // Database Metrics
    pub static ref DB_POOL_SIZE: Gauge =
        Gauge::with_opts(Opts::new("shortly_database_connection_pool_size",
            "Database connection pool size"))
        .expect("metric creation");

    pub static ref DB_POOL_IDLE: Gauge =
        Gauge::with_opts(Opts::new("shortly_database_connection_pool_idle",
            "Idle database connections"))
        .expect("metric creation");

    // System Metrics
    pub static ref UPTIME: Gauge =
        Gauge::with_opts(Opts::new("shortly_uptime_seconds",
            "Application uptime in seconds"))
        .expect("metric creation");

    pub static ref VERSION_INFO: GaugeVec =
        GaugeVec::new(
            Opts::new("shortly_version_info", "Application version information"),
            &["version"]
        ).expect("metric creation");

    // TTL Histogram
    pub static ref URLS_TTL_HISTOGRAM: Histogram =
        Histogram::with_opts(
            HistogramOpts::new("shortly_urls_ttl_hours",
                "Distribution of URL TTL in hours")
            .buckets(vec![1.0, 6.0, 12.0, 24.0, 48.0, 72.0, 168.0, 336.0, 720.0])
        ).expect("metric creation");
}

pub fn register_metrics() {
    REGISTRY
        .register(Box::new(URLS_TOTAL.clone()))
        .expect("register");
    REGISTRY
        .register(Box::new(URLS_LAST_CREATED.clone()))
        .expect("register");
    REGISTRY
        .register(Box::new(URLS_CUSTOM_NAMED.clone()))
        .expect("register");
    REGISTRY
        .register(Box::new(URLS_EXPIRED.clone()))
        .expect("register");
    REGISTRY
        .register(Box::new(URLS_LAST_ACCESSED.clone()))
        .expect("register");
    REGISTRY
        .register(Box::new(URLS_DELETED_24H.clone()))
        .expect("register");
    REGISTRY
        .register(Box::new(USERS_TOTAL.clone()))
        .expect("register");
    REGISTRY
        .register(Box::new(USERS_ACTIVE_SESSIONS.clone()))
        .expect("register");
    REGISTRY
        .register(Box::new(USERS_LAST_LOGIN.clone()))
        .expect("register");
    REGISTRY
        .register(Box::new(AUDIT_EVENTS_TOTAL.clone()))
        .expect("register");
    REGISTRY
        .register(Box::new(AUDIT_LAST_EVENT.clone()))
        .expect("register");
    REGISTRY
        .register(Box::new(DB_POOL_SIZE.clone()))
        .expect("register");
    REGISTRY
        .register(Box::new(DB_POOL_IDLE.clone()))
        .expect("register");
    REGISTRY
        .register(Box::new(UPTIME.clone()))
        .expect("register");
    REGISTRY
        .register(Box::new(VERSION_INFO.clone()))
        .expect("register");
    REGISTRY
        .register(Box::new(URLS_TTL_HISTOGRAM.clone()))
        .expect("register");
}
