use super::asset::Asset;
use super::email::Email;
use super::feature::Feature;
use super::frame::FrameId;
use super::frame_name::FrameName;
use super::locale::Locale;
use super::time_zone::TimeZone;
use super::url::Url;
use super::user::UserId;
use super::user_name::UserName;
use chrono::DateTime;
use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct FeedItem {
    pub assets: Vec<Asset>,
    pub metadata: Value,
    pub message: Value,
    pub stick_for: Value,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FeedItemMetadata {
    pub attribution: String,
    pub date: String,
    pub location: Value,
    pub pair_reasons: Value,
}
