use std::rc::Rc;

use chrono::DateTime;
use chrono::Utc;
use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;
use vscodehelper_macros::StringHolder;

use super::email::Email;
use super::feature::Feature;
use super::frame::FrameId;
use super::frame_name::FrameName;
use super::locale::Locale;
use super::time_zone::TimeZone;
use super::url::Url;
use super::user_id::UserId;
use super::user_name::UserName;

#[derive(StringHolder)]
pub struct FrameEnvironmentId {
    pub inner: Rc<str>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FrameEnvironment {
    pub id: FrameEnvironmentId,
    pub frame_id: FrameId,
    pub last_online_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
