use super::frame::FrameId;
use chrono::DateTime;
use chrono::Utc;
use serde::Deserialize;
use serde::Serialize;
use std::sync::Arc;
use holda::StringHolda;

#[derive(StringHolda)]
pub struct FrameEnvironmentId {
    pub inner: Arc<str>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FrameEnvironment {
    pub id: FrameEnvironmentId,
    pub frame_id: FrameId,
    pub last_online_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
