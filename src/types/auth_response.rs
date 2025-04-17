use crate::types::auth_token::AuthToken;
use crate::types::email::Email;
use crate::types::feature::Feature;
use crate::types::file_name::FileName;
use crate::types::locale::Locale;
use crate::types::user_name::UserName;
use chrono::DateTime;
use chrono::Utc;
use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;

use super::user::UserId;

#[derive(Serialize, Deserialize)]
pub struct AuthResponse {
    pub error: bool,
    pub result: AuthResponseResult,
}
#[derive(Serialize, Deserialize)]
pub struct AuthResponseResult {
    pub current_user: AuthResponseResultCurrentUser,
}

#[derive(Serialize, Deserialize)]
pub struct AuthResponseResultCurrentUser {
    pub admin_account: Option<Value>,
    pub analytics_optout: bool,
    pub attribution_id: Option<Value>,
    pub attribution_string: Option<Value>,
    pub auth_token: AuthToken,
    pub auto_upload_off: bool,
    pub avatar_file_name: FileName,
    pub charity_subscriptions_launched: bool,
    pub confirmed_email: Email,
    pub created_at: DateTime<Utc>,
    pub current_source_id: String,
    pub eligible_for_app_review_prompt: bool,
    pub email: Email,
    pub features: Vec<Feature>,
    pub google_photos_disabled: Option<Value>,
    pub has_access_to_new_google_photos: bool,
    pub has_frame: bool,
    pub id: UserId,
    pub in_app_promo_opt_out: Option<Value>,
    pub latest_app_version: Option<Value>,
    pub live_photos_launched: bool,
    pub locale: Locale,
    pub name: UserName,
    pub short_id: Option<Value>,
    pub show_push_prompt: bool,
    pub smart_albums_off: bool,
    pub smart_suggestions_off: bool,
    pub standard_account_texter: Value,
    pub subscriptions_launched: bool,
    pub test_account: Option<Value>,
    pub thanks_launched: bool,
    pub tooltip_add_photos_seen: bool,
    pub tooltip_added_photos_seen: bool,
    pub tooltip_gestures_seen: bool,
    pub tooltip_inbox_seen: bool,
    pub tooltip_manage_frames_seen: bool,
    pub tooltip_settings_seen: bool,
    pub unconfirmed_email: Option<Value>,
    pub updated_at: DateTime<Utc>,
    pub verbose_logging_enabled: bool,
    pub warn_smart_albums_deprecated: bool,
}
