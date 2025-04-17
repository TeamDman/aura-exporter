use super::email::Email;
use super::user_name::UserName;
use chrono::DateTime;
use chrono::Utc;
use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;
use std::rc::Rc;
use vscodehelper_macros::StringHolder;

#[derive(StringHolder)]
pub struct UserId {
    pub inner: Rc<str>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: UserId,
    pub short_id: Value,
    pub test_account: Value,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub latest_app_version: Value,
    pub name: UserName,
    pub email: Email,
    pub attribution_id: Value,
    pub attribution_string: Value,
    pub show_push_prompt: bool,
    pub avatar_file_name: Value,
    pub analytics_optout: bool,
}
