use chrono::DateTime;
use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;

use super::email::Email;
use super::feature::Feature;
use super::frame::FrameId;
use super::frame_name::FrameName;
use super::locale::Locale;
use super::time_zone::TimeZone;
use super::url::Url;
use super::user_id::UserId;
use super::user_name::UserName;

#[derive(Debug, Serialize, Deserialize)]
pub struct Padding {
    pub top: f32,
    pub right: f32,
    pub bottom: f32,
    pub left: f32,
}
