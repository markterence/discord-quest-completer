use discord_sdk::activity::{ActivityBuilder, ActivityKind};
use std::{fmt::Error, io::ErrorKind, ops::Deref};

use crate::rpc::{self, Client};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct ActivityParams {
    pub app_id: String,
    pub details: Option<String>,
    pub state: Option<String>,
    #[serde(rename = "largeImageKey")]
    pub large_image_key: Option<String>,
    #[serde(rename = "largeImageText")]
    pub large_image_text: Option<String>,
    pub timestamp: Option<i64>,
    pub activity_kind: Option<i32>,
}

pub struct CreateActivityResult {
    pub activity: ActivityBuilder,
    pub app_id: u64,
}

fn to_app_id(app_id: &str) -> Result<u64, std::num::ParseIntError> {
    app_id.parse::<u64>().map_err(|e| {
        eprintln!("Failed to parse app_id: {}", e);
        std::num::ParseIntError::from(e)
    })
}

pub fn parse_activity_json(activity_json: &str) -> Result<ActivityParams, String> {
    serde_json::from_str(activity_json).map_err(|e| {
        eprintln!("Failed to parse activity JSON: {}", e);
        format!("Failed to parse activity JSON: {}", e)
    })
}

pub fn create_activity(activity_json: String) -> Result<CreateActivityResult, String> {
    let activity: ActivityParams = parse_activity_json(&activity_json)?;

    let app_id: u64 = to_app_id(&activity.app_id).map_err(|e| {
        eprintln!("Failed to parse app_id: {}", e);
        format!("Failed to parse app_id: {}", e)
    })?;

    let details = activity.details.unwrap_or_default();
    let state = activity.state.unwrap_or_default();
    let large_image_key = activity.large_image_key.unwrap_or_default();
    let large_image_text = activity.large_image_text;
    let timestamp = activity.timestamp;
    let activity_kind = activity.activity_kind.unwrap_or(0);

    let mut rp: discord_sdk::activity::ActivityBuilder =
        rpc::ds::activity::ActivityBuilder::default();

    if Some(activity_kind) != None {
        if activity_kind == 0 {
            rp = rp.kind(rpc::ds::activity::ActivityKind::Playing);
        } else if activity_kind == 2 {
            rp = rp.kind(rpc::ds::activity::ActivityKind::Listening);
        } else if activity_kind == 3 {
            rp = rp.kind(rpc::ds::activity::ActivityKind::Watching);
        } else if activity_kind == 5 {
            rp = rp.kind(rpc::ds::activity::ActivityKind::Competing);
        } else {
            rp = rp.kind(rpc::ds::activity::ActivityKind::Playing);
        }
    }

    // details
    if !details.is_empty() {
        rp = rp.details(details);
    }

    // state
    if !state.is_empty() {
        rp = rp.state(state);
    }

    // timestamp
    if let Some(ts) = timestamp {
        rp = rp.start_timestamp(ts as i64);
    }

    // large_image_key
    if !large_image_key.is_empty() {
        rp = rp
            .assets(rpc::ds::activity::Assets::default().large(&large_image_key, large_image_text));
    }

    Ok(CreateActivityResult {
        activity: rp,
        app_id: app_id,
    })
}

pub async fn set_activity(activity_json: String) -> Result<Client, String> {
    let activity_result: CreateActivityResult = create_activity(activity_json)?;
    let app_id: i64 = activity_result.app_id as i64;
    let activity_builder = activity_result.activity;

    let client = rpc::make_client(app_id, rpc::ds::Subscriptions::ACTIVITY).await;
    client
        .discord
        .update_activity(activity_builder)
        .await
        .map_err(|e| format!("Failed to update activity: {}", e))?;

    Ok(client)
}
