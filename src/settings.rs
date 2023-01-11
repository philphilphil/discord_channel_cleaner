use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Settings {
    pub discord: Discord,
    pub cleaning: Cleaning,
    #[serde(alias = "channel")]
    pub channels: Vec<Channel>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Discord {
    pub token: String,
    pub application_id: u64,
    pub guild_id: u32,
    pub admin_channel_id: u64,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Cleaning {
    pub image_cdn_allowlist: Vec<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Rule {
    pub name: String,
    pub rule_set: Vec<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Channel {
    pub id: u64,
    pub keep_images: bool,
    pub max_message_age_in_h: u32,
}
