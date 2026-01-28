use std::sync::Arc;

use jiff::Timestamp;

#[derive(Debug, Clone)]
pub struct AppState {
    pub shared_state: Arc<AppStateInner>,
}

impl AppState {
    pub fn init() -> Self {
        let state = AppStateInner {
            start_time: Timestamp::now(),
        };
        Self {
            shared_state: Arc::new(state),
        }
    }
}

#[derive(Debug, Clone)]
pub struct AppStateInner {
    pub start_time: Timestamp,
}
