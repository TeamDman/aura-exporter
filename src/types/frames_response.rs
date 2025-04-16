use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;

use super::frame::Frame;

#[derive(Debug, Serialize, Deserialize)]
pub struct FramesResponse {
    pub frames: Vec<Frame>,
    pub user_pending_tokens: Vec<Value>,
}
