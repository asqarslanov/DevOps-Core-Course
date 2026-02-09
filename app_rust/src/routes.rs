use std::borrow::Cow;
use std::net::SocketAddr;

use axum::extract::{ConnectInfo, State};
use axum::routing::get;
use axum::{Json, Router};
use axum_extra::TypedHeader;
use headers::UserAgent;
use http::Uri;
use jiff::Timestamp;
use serde::{Deserialize, Serialize};

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
            name: Cow::Borrowed(env!("CARGO_PKG_NAME")),
            version: Cow::Borrowed(env!("CARGO_PKG_VERSION")),
            description: Cow::Borrowed(env!("CARGO_PKG_DESCRIPTION")),
            framework: Cow::Borrowed("axum"),
        },
        system: SystemInfo {
            hostname: sysinfo::System::host_name(),
            platform: sysinfo::System::name(),
            platform_version: sysinfo::System::os_version(),
            architecture: sysinfo::System::cpu_arch(),
            cpu_count: sys.cpus().len(),
            rust_version: Cow::Borrowed("1.93"),
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
                path: Cow::Borrowed("/"),
                method: http::Method::GET,
                description: Cow::Borrowed("Service information"),
            },
            EndpointInfo {
                path: Cow::Borrowed("/health"),
                method: http::Method::GET,
                description: Cow::Borrowed("Health check"),
            },
        ],
    };

    Json(response)
}

#[derive(Debug, Serialize, Deserialize)]
struct GetResponse {
    service: ServiceInfo,
    system: SystemInfo,
    runtime: RuntimeInfo,
    request: RequestInfo,
    endpoints: Vec<EndpointInfo>,
}

#[derive(Debug, Serialize, Deserialize)]
struct ServiceInfo {
    name: Cow<'static, str>,
    version: Cow<'static, str>,
    description: Cow<'static, str>,
    framework: Cow<'static, str>,
}

#[derive(Debug, Serialize, Deserialize)]
struct SystemInfo {
    hostname: Option<String>,
    platform: Option<String>,
    platform_version: Option<String>,
    architecture: String,
    cpu_count: usize,
    rust_version: Cow<'static, str>,
}

#[derive(Debug, Serialize, Deserialize)]
struct RuntimeInfo {
    uptime_seconds: i64,
    uptime_human: String,
    current_time: Timestamp,
    timezone: Timezone,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
enum Timezone {
    Utc,
}

#[derive(Debug, Serialize, Deserialize)]
struct RequestInfo {
    client_ip: SocketAddr,

    user_agent: String,

    #[serde(with = "http_serde::method")]
    method: http::Method,

    #[serde(with = "http_serde::uri")]
    path: Uri,
}

#[derive(Debug, Serialize, Deserialize)]
struct EndpointInfo {
    path: Cow<'static, str>,

    #[serde(with = "http_serde::method")]
    method: http::Method,

    description: Cow<'static, str>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn get() {
        let server = crate::test_server();

        let response = server
            .get("/")
            .add_header("User-Agent", <&str>::default())
            .await;

        response.assert_status_ok();
        _ = response.json::<GetResponse>();
    }
}
