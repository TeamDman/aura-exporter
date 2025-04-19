use crate::assets::get_assets_for_frame;
use crate::frames::get_frames;
use crate::remote_types::frame::FrameId;
use crate::remote_types::user_name::UserName;
use itertools::Itertools;
use std::collections::HashMap;

pub async fn summarize_assets_for_frame(frame_id: &FrameId) -> eyre::Result<()> {
    let frame_assets = get_assets_for_frame(frame_id).await?;
    let frames = get_frames().await?;
    let users = frames
        .frames
        .iter()
        .flat_map(|frame| &frame.contributors)
        .map(|user| (user.id.clone(), user))
        .collect::<HashMap<_, _>>();
    let asset_users = frame_assets.assets.iter().counts_by(|asset| &asset.user_id);
    let longest_name_length = users
        .values()
        .filter(|user| asset_users.contains_key(&user.id))
        .map(|user| user.name.to_string().len())
        .max()
        .unwrap_or(0);
    for (user_id, count) in asset_users.into_iter().sorted_by_key(|(_, count)| *count) {
        let user_display = users
            .get(user_id)
            .map(|user| user.name.clone())
            .unwrap_or(UserName::new("Unknown User"));
        println!(
            "{} {:>width$}\t{}",
            user_id,
            user_display.to_string(),
            count,
            width = longest_name_length
        );
    }
    Ok(())
}
