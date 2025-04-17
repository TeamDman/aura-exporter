use super::frame::Frame;
use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct FramesResponse {
    pub frames: Vec<Frame>,
    pub user_pending_tokens: Vec<Value>,
}
