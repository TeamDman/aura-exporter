use std::rc::Rc;

use super::file_name::FileName;
use super::padding::Padding;
use super::url::Url;
use super::user::User;
use super::user::UserId;
use chrono::DateTime;
use chrono::Utc;
use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;
use vscodehelper_macros::StringHolder;

#[derive(StringHolder)]
pub struct AssetId {
    pub inner: Rc<str>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Asset {
    pub id: AssetId,
    pub user_id: UserId,
    pub thumbnail_url: Url,
    pub portrait_url: Option<Url>,
    pub landscape_url: Option<Url>,
    pub widget_url: Url,
    pub minibar_url: Url,
    pub minibar_landscape_url: Option<Url>,
    pub minibar_portrait_url: Url,
    pub landscape_16_10_url: Option<Url>,
    pub portrait_4_5_url: Url,
    pub portrait_url_padding: Value,
    pub landscape_url_padding: Value,
    pub landscape_16_10_url_padding: Value,
    pub portrait_4_5_url_padding: Option<Padding>,
    pub video_url: Value,
    pub landscape_rect: Value,
    pub portrait_rect: Value,
    pub user_landscape_rect: Value,
    pub user_portrait_rect: Value,
    pub auto_landscape_16_10_rect: Value,
    pub user_landscape_16_10_rect: Value,
    pub auto_portrait_4_5_rect: Value,
    pub user_portrait_4_5_rect: Value,
    pub exif_orientation: i64,
    pub handled_at: DateTime<Utc>,
    pub uploaded_at: DateTime<Utc>,
    pub good_resolution: bool,
    pub source_id: String,
    pub duplicate_of_id: Value,
    pub rotation_cw: i64,
    pub md5_hash: Option<String>,
    pub is_subscription: bool,
    pub glaciered_at: DateTime<Utc>,
    pub unglacierable: Option<bool>,
    pub duration: Value,
    pub live_photo_off: Value,
    pub attachments: Vec<Value>,
    pub local_identifier: String,
    pub created_at_on_client: Value,
    pub selected: bool,
    pub file_name: FileName,
    pub raw_file_name: Value,
    pub video_file_name: Value,
    pub colorized_file_name: Value,
    pub width: i64,
    pub height: i64,
    pub taken_at: DateTime<Utc>,
    pub modified_at: DateTime<Utc>,
    pub horizontal_accuracy: Value,
    pub favorite: Value,
    pub orientation: i64,
    pub hdr: Value,
    pub panorama: Value,
    pub is_live: Value,
    pub burst_id: Value,
    pub burst_selection_types: Value,
    pub represents_burst: Value,
    pub data_uti: FileName,
    pub original_file_name: Option<String>,
    pub upload_priority: i64,
    pub ios_media_subtypes: Value,
    pub taken_at_user_override_at: Value,
    pub taken_at_granularity: Value,
    pub duration_unclipped: Value,
    pub video_clip_start: Value,
    pub video_clip_excludes_audio: Value,
    pub video_clipped_by_user_at: Value,
    pub location: Value,
    pub user: User,
}

impl Asset {
    pub fn get_url(&self) -> Url {
        Url::new(format!(
            "https://imgproxy.pushd.com/{user_id}/{file_name}",
            user_id = self.user_id,
            file_name = self.file_name
        ))
    }
}
