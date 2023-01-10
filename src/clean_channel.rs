use serenity::{
    futures::StreamExt,
    http::Http,
    model::{channel::Message, id::ChannelId},
    prelude::*,
};
use std::time::SystemTime;
use tracing::{error, info};

use crate::settings::Settings;

pub async fn execute(settings: Settings) {
    // connect to api and clean
    info!("Starting clean job.");
    let intents = GatewayIntents::all();

    let admin_channel = ChannelId(settings.discord.admin_channel_id as u64);

    let client = Client::builder(settings.discord.token, intents)
        .await
        .expect("Err creating client");
    let ctx = &client.cache_and_http.http;

    for c in settings.channels {
        info!(c.id);
        continue;
        let channel = ChannelId(c.id as u64);
        let channel_name = channel
            .to_channel(&client.cache_and_http)
            .await
            .unwrap()
            .guild()
            .unwrap()
            .name; // for some reason .name() on ChannelId does not work.

        let (purge_count, count_media_kept) = purge_channel(&channel, ctx).await;

        let delete_count_msg = format!(
            "Purge #{}: Deleted {} messages, kept {} images.",
            channel_name, purge_count, count_media_kept
        );

        admin_channel.say(ctx, &delete_count_msg).await.unwrap();
        info!("{}", &delete_count_msg);
    }
}

async fn purge_channel(channel: &ChannelId, ctx: &Http) -> (u64, u64) {
    let mut count_deleted = 0;
    let mut count_media_kept = 0;

    let mut messages = channel.messages_iter(&ctx).boxed();

    while let Some(message_result) = messages.next().await {
        match message_result {
            Ok(message) => {
                if !message.attachments.is_empty() || linked_image(&message) {
                    count_media_kept += 1;
                } else if message_older_then_one_day(&message) {
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

fn linked_image(msg: &Message) -> bool {
    if !msg.content.contains("http") {
        return false;
    }

    // TODO: move to config
    let allowed_urls = vec![
        "instagram.com",
        "imgur.com",
        "cdn.discordapp.com",
        "media.jipvankuijk.nl",
    ];

    for url in allowed_urls {
        if msg.content.contains(url) {
            return true;
        }
    }

    false
}

fn message_older_then_one_day(msg: &Message) -> bool {
    let msg_unix = msg.timestamp.unix_timestamp() as u64;
    let current_unix = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs();

    (current_unix - msg_unix) > 86400
}
