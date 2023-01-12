use anyhow::Result;
use serenity::{
    futures::StreamExt,
    http::Http,
    model::{channel::Message, id::ChannelId},
    prelude::*,
};
use std::time::SystemTime;
use tracing::{error, info};

use crate::settings::{Channel, Settings};

pub async fn execute(settings: Settings) -> Result<()> {
    info!("Start cleaning {} channels.", settings.channels.len());
    let intents = GatewayIntents::all();
    let admin_channel = ChannelId(settings.discord.admin_channel_id);
    let client = Client::builder(settings.discord.token, intents)
        .await
        .expect("Err creating client");
    let ctx = &client.cache_and_http.http;

    for c in settings.channels {
        let channel = ChannelId(c.id);

        let channel_name = channel
            .to_channel(&client.cache_and_http)
            .await?
            .guild()
            .unwrap()
            .name; // for some reason .name() on ChannelId does not work.

        let (purge_count, media_kept_count) =
            purge_channel(&c, &settings.cleaning.image_cdn_allowlist, ctx).await;

        let delete_count_msg = build_delete_count_msg(channel_name, purge_count, media_kept_count);

        admin_channel
            .say(ctx, &delete_count_msg)
            .await
            .expect("Error sending message to admin channel.");

        info!("{}", &delete_count_msg);
    }

    info!("Done.");
    Ok(())
}

async fn purge_channel(channel: &Channel, allow_list: &Vec<String>, ctx: &Http) -> (u64, u64) {
    let channel_id = ChannelId(channel.id);
    let mut count_deleted = 0;
    let mut count_media_kept = 0;

    let mut messages = channel_id.messages_iter(&ctx).boxed();

    while let Some(message_result) = messages.next().await {
        match message_result {
            Ok(message) => {
                if channel.keep_images
                    && (!message.attachments.is_empty() || linked_image(&message, allow_list))
                {
                    count_media_kept += 1;
                } else if message_older_then_x_hours(&message, channel.max_message_age_in_h) {
                    match message.delete(&ctx).await {
                        Ok(_) => count_deleted += 1,
                        Err(error) => {
                            error!("Error deleting msg: {}. Error: {}", message.id, error)
                        }
                    }
                }
            }
            Err(error) => error!("Error fetching messages: {}", error),
        }
    }

    (count_deleted, count_media_kept)
}

fn linked_image(msg: &Message, allow_list: &Vec<String>) -> bool {
    if !msg.content.contains("http") {
        return false;
    }

    for url in allow_list {
        if msg.content.contains(url) {
            return true;
        }
    }

    false
}

fn message_older_then_x_hours(msg: &Message, hours: u32) -> bool {
    let msg_unix = msg.timestamp.unix_timestamp() as u64;
    let current_unix = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs();

    (current_unix - msg_unix) > hours as u64 * 60 * 60
}

fn build_delete_count_msg(channel_name: String, purge_count: u64, media_kept_count: u64) -> String {
    let mut msg = format!("Clean #{}: ", channel_name);

    if purge_count > 0 {
        msg += &format!("Deleted {} messages.", purge_count);
    } else {
        msg += "Deleted no messages.";
    }

    if media_kept_count > 0 {
        msg += &format!(" Kept {} images.", media_kept_count);
    }

    msg
}
