use serenity::model::prelude::ChannelId;

pub fn str_to_channel_id(id_as_str: &str) -> ChannelId {
    let channel_id: u64 = id_as_str.parse().expect("Error parsing purge channel id.");
    ChannelId(channel_id)
}
