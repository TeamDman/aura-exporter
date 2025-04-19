use super::asset::Asset;
use super::asset_setting::AssetSetting;
use super::user::User;
use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Serialize, Deserialize)]
pub struct FrameAssetsResponse {
    pub asset_settings: Vec<AssetSetting>,
    pub assets: Vec<Asset>,
    pub users: Vec<User>,
}
