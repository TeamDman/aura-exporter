use super::asset::Asset;
use super::asset::AssetId;
use super::frame::FrameId;
use chrono::DateTime;
use chrono::Utc;
use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;
use std::sync::Arc;
use holda::StringHolda;

#[derive(StringHolda)]
pub struct ImpressionId {
    pub inner: Arc<str>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Impression {
    pub last_viewed_or_created_at: DateTime<Utc>,
    pub view_count: i64,
    pub gesture_direction: Value,
    pub created_at: DateTime<Utc>,
    pub live_photo_on_transition: Value,
    pub viewed_at: DateTime<Utc>,
    pub id: ImpressionId,
    pub last_viewed_at: DateTime<Utc>,
    pub last_shown_with_asset_id: Value,
    pub frame_id: FrameId,
    pub asset_id: AssetId,
    pub asset: Asset,
}
