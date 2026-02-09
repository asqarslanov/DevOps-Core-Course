use axum::extract::State;
use axum::routing::get;
use axum::{Json, Router};
use jiff::Timestamp;
use serde::{Deserialize, Serialize};

use crate::state::AppState;

pub fn router() -> Router<AppState> {
    Router::new().route("/", get(handle_get))
}

async fn handle_get(State(state): State<AppState>) -> Json<GetResponse> {
    let now = Timestamp::now();
    let response = GetResponse {
        status: HealthStatus::Healthy,
        timestamp: now,
        uptime_seconds: (now - state.shared_state.start_time).get_seconds(),
    };

    Json(response)
}

#[derive(Debug, Serialize, Deserialize)]
struct GetResponse {
    status: HealthStatus,
    timestamp: Timestamp,
    uptime_seconds: i64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
enum HealthStatus {
    Healthy,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn get() {
        let server = crate::test_server();

        let response = server.get("/health").await;

        response.assert_status_ok();
        _ = response.json::<GetResponse>();
    }
}
