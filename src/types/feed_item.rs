use super::asset::Asset;
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
