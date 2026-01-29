use std::net::SocketAddr;

use axum::extract::{ConnectInfo, State};
use axum::routing::get;
use axum::{Json, Router};
use axum_extra::TypedHeader;
use headers::UserAgent;
use http::Uri;
use jiff::Timestamp;
use serde::Serialize;

use crate::state::AppState;

mod health;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", get(handle_get))
        .nest("/health", health::router())
}

async fn handle_get(
    State(state): State<AppState>,
    ConnectInfo(client_ip): ConnectInfo<SocketAddr>,
    TypedHeader(user_agent): TypedHeader<UserAgent>,
    method: http::Method,
    uri: Uri,
) -> Json<GetResponse> {
    let sys = sysinfo::System::new_all();
    let now = Timestamp::now();
    let uptime = now - state.shared_state.start_time;
    let uptime_human = format!(
        "{} hours, {} minutes",
        uptime.get_hours(),
        uptime.get_minutes(),
    );

    let response = GetResponse {
        service: ServiceInfo {
            name: env!("CARGO_PKG_NAME"),
            version: env!("CARGO_PKG_VERSION"),
            description: env!("CARGO_PKG_DESCRIPTION"),
            framework: "axum",
        },
        system: SystemInfo {
            hostname: sysinfo::System::host_name(),
            platform: sysinfo::System::name(),
            platform_version: sysinfo::System::os_version(),
            architecture: sysinfo::System::cpu_arch(),
            cpu_count: sys.cpus().len(),
            rust_version: "1.93",
        },
        runtime: RuntimeInfo {
            uptime_seconds: uptime.get_seconds(),
            uptime_human,
            current_time: now,
            timezone: Timezone::Utc,
        },
        request: RequestInfo {
            client_ip,
            user_agent: user_agent.to_string(),
            method,
            path: uri,
        },
        endpoints: vec![
            EndpointInfo {
                path: "/",
                method: http::Method::GET,
                description: "Service information",
            },
            EndpointInfo {
                path: "/health",
                method: http::Method::GET,
                description: "Health check",
            },
        ],
    };

    Json(response)
}

#[derive(Debug, Serialize)]
struct GetResponse {
    service: ServiceInfo,
    system: SystemInfo,
    runtime: RuntimeInfo,
    request: RequestInfo,
    endpoints: Vec<EndpointInfo>,
}

#[derive(Debug, Serialize)]
struct ServiceInfo {
    name: &'static str,
    version: &'static str,
    description: &'static str,
    framework: &'static str,
}

#[derive(Debug, Serialize)]
struct SystemInfo {
    hostname: Option<String>,
    platform: Option<String>,
    platform_version: Option<String>,
    architecture: String,
    cpu_count: usize,
    rust_version: &'static str,
}

#[derive(Debug, Serialize)]
struct RuntimeInfo {
    uptime_seconds: i64,
    uptime_human: String,
    current_time: Timestamp,
    timezone: Timezone,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "UPPERCASE")]
enum Timezone {
    Utc,
}

#[derive(Debug, Serialize)]
struct RequestInfo {
    client_ip: SocketAddr,

    user_agent: String,

    #[serde(with = "http_serde::method")]
    method: http::Method,

    #[serde(with = "http_serde::uri")]
    path: Uri,
}

#[derive(Debug, Serialize)]
struct EndpointInfo {
    path: &'static str,

    #[serde(with = "http_serde::method")]
    method: http::Method,

    description: &'static str,
}
