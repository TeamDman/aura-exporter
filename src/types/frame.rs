use std::rc::Rc;

use chrono::DateTime;
use chrono::Utc;
use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;
use vscodehelper_macros::StringHolder;

use super::asset::Asset;
use super::delivered_frame_gift::DeliveredFrameGift;
use super::email::Email;
use super::feature::Feature;
use super::feed_item::FeedItem;
use super::frame_environment::FrameEnvironment;
use super::frame_name::FrameName;
use super::impression::Impression;
use super::locale::Locale;
use super::time_zone::TimeZone;
use super::url::Url;
use super::user::User;
use super::user::UserId;

#[derive(StringHolder)]
pub struct FrameId {
    pub inner: Rc<str>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Frame {
    pub id: FrameId,
    pub name: FrameName,
    pub user_id: UserId,
    pub software_version: String,
    pub build_version: String,
    pub hw_android_version: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub handled_at: DateTime<Utc>,
    pub deleted_at: Value,
    pub updated_at_on_client: Value,
    pub last_impression_at: DateTime<Utc>,
    pub orientation: i64,
    pub auto_brightness: bool,
    pub min_brightness: i64,
    pub max_brightness: i64,
    pub brightness: Value,
    pub sense_motion: bool,
    pub default_speed: Value,
    pub slideshow_interval: i64,
    pub slideshow_auto: bool,
    pub digits: i64,
    pub contributor_tokens: Vec<Value>,
    pub hw_serial: String,
    pub matting_color: String,
    pub trim_color: String,
    pub is_handling: bool,
    pub calibrations_last_modified_at: DateTime<Utc>,
    pub gestures_on: bool,
    pub portrait_pairing_off: Value,
    pub live_photos_on: bool,
    pub auto_processed_playlist_ids: Vec<Value>,
    pub time_zone: TimeZone,
    pub wifi_network: String,
    pub cold_boot_at: Value,
    pub is_charity_water_frame: bool,
    pub num_assets: i64,
    pub thanks_on: bool,
    pub frame_queue_url: Option<Url>,
    pub client_queue_url: Url,
    pub scheduled_display_sleep: bool,
    pub scheduled_display_on_at: Value,
    pub scheduled_display_off_at: Value,
    pub forced_wifi_state: Value,
    pub forced_wifi_recipient_email: Value,
    pub is_analog_frame: bool,
    pub control_type: String,
    pub display_aspect_ratio: String,
    pub has_claimable_gift: Value,
    pub gift_billing_hint: Value,
    pub locale: Locale,
    pub frame_type: Value,
    pub description: Value,
    pub representative_asset_id: Value,
    pub sort_mode: Option<String>,
    pub email_address: Email,
    pub features: Vec<Feature>,
    pub volume: i64,
    pub letterbox_style: Value,
    pub pitch: Value,
    pub wifi_frequency: i64,
    pub attachment_caption_display: bool,
    pub user: User,
    pub playlists: Vec<Value>,
    pub delivered_frame_gift: Option<DeliveredFrameGift>,
    pub last_feed_item: FeedItem,
    pub last_impression: Impression,
    pub recent_assets: Vec<Asset>,
    pub contributors: Vec<User>,
    pub frame_environment: FrameEnvironment,
    pub child_albums: Vec<Value>,
    pub smart_adds: Vec<Value>,
}
