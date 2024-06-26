use std::error::Error;

use reqwest::Client;
use serde_json::Value;

pub async fn request_scene_layer_info(
    client: &Client,
) -> Result<serde_json::Value, Box<dyn Error>> {
    let resp = client.get("layers/0").send().await?;
    let val = resp.json::<Value>().await?;
    Ok(val)
}
