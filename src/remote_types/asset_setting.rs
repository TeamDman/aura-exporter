use super::asset::AssetId;
use super::frame::FrameId;
use super::user::UserId;
use chrono::DateTime;
use chrono::Utc;
use serde::Deserialize;
use serde::Serialize;
use std::sync::Arc;
use holda::StringHolda;

#[derive(StringHolda)]
pub struct AssetSettingId {
    pub inner: Arc<str>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssetSetting {
    pub added_by_id: UserId,
    pub asset_id: AssetId,
    pub created_at: DateTime<Utc>,
    pub frame_id: FrameId,
    pub hidden: bool,
    pub id: AssetSettingId,
    pub last_impression_at: Option<DateTime<Utc>>,
    pub reason: String,
    pub selected: bool,
    pub updated_at: Option<DateTime<Utc>>,
    pub updated_selected_at: DateTime<Utc>,
}
