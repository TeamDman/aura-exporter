use super::email::Email;
use super::frame::FrameId;
use super::user::User;
use super::user::UserId;
use super::user_name::UserName;
use chrono::DateTime;
use chrono::Utc;
use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;
use std::sync::Arc;
use holda::StringHolda;

#[derive(StringHolda)]
pub struct DeliveredFrameGiftId {
    pub inner: Arc<str>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeliveredFrameGift {
    pub id: DeliveredFrameGiftId,
    pub order_item_id: Value,
    pub order_item_fulfilled_hw_serial: Value,
    pub frame_id: FrameId,
    pub gift_message: Value,
    pub selected_asset_id: Value,
    pub user_id: UserId,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub recipient_name: UserName,
    pub recipient_email: Email,
    pub is_tweed_wolf: bool,
    pub delivered_at: DateTime<Utc>,
    pub skipped_message: bool,
    pub skipped_invites: bool,
    pub skipped_photos: bool,
    pub skipped_wifi: Value,
    pub shipped_to: Value,
    pub claim_code: Value,
    pub claim_code_recipient_email: Value,
    pub highlight_claim_code: bool,
    pub claim_code_reminder_date: Value,
    pub order: Value,
    pub order_item: Value,
    pub selected_asset: Value,
    pub user: User,
}
