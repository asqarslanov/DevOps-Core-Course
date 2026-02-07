use std::net::{SocketAddr, SocketAddrV4};

use axum::extract::{MatchedPath, Request};
use const_format::formatcp;
use tokio::net::TcpListener;
use tower_http::trace::TraceLayer;
use tracing::{info, info_span};
use tracing_subscriber::EnvFilter;
use tracing_subscriber::layer::SubscriberExt as _;
use tracing_subscriber::util::SubscriberInitExt as _;

use crate::config::AppConfig;
use crate::state::AppState;

mod config;
mod routes;
mod state;

#[tokio::main]
async fn main() -> eyre::Result<()> {
    _ = dotenvy::dotenv();

    let config = AppConfig::load()?;
    init_logging(&config);

    info!("Debug mode: {}", config.debug);

    let state = AppState::init();

    let addr = SocketAddrV4::new(config.host, config.port);
    let listener = TcpListener::bind(addr).await?;

    let router =
        self::routes::router()
            .with_state(state)
            .layer(
                TraceLayer::new_for_http().make_span_with(|request: &Request<_>| {
                    let matched_path = request
                        .extensions()
                        .get::<MatchedPath>()
                        .map(MatchedPath::as_str);

                    info_span!(
                        "http_request",
                        method = ?request.method(),
                        matched_path,
                        some_other_field = tracing::field::Empty,
                    )
                }),
            );

    info!("Listening on {addr}");
    axum::serve(
        listener,
        router.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await?;

    Ok(())
}

fn init_logging(config: &AppConfig) {
    let env_filter = {
        EnvFilter::try_from_default_env().unwrap_or_else(|_| {
            EnvFilter::new(if config.debug {
                formatcp!(
                    "{}=debug,tower_http=debug,axum::rejection=trace",
                    env!("CARGO_CRATE_NAME"),
                )
            } else {
                formatcp!(
                    "{}=info,tower_http=debug,axum::rejection=trace",
                    env!("CARGO_CRATE_NAME"),
                )
            })
        })
    };

    let fmt_layer = {
        tracing_subscriber::fmt::layer()
            .pretty()
            .with_file(false)
            .with_line_number(false)
            .with_target(false)
            .without_time()
    };

    tracing_subscriber::registry()
        .with(env_filter)
        .with(fmt_layer)
        .init();
}

#[cfg(test)]
fn test_server() -> axum_test::TestServer {
    axum_test::TestServer::new(
        routes::router()
            .with_state(AppState::init())
            .into_make_service_with_connect_info::<SocketAddr>(),
    )
    .unwrap()
}
