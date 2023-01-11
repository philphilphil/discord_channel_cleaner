mod clean_channel;
mod settings;

use std::env;

use config::Config;
use tracing::Level;
use tracing_subscriber::FmtSubscriber;

#[tokio::main]
async fn main() {
    // Log/Output settings
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    let mut settings_location = "settings.toml";

    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        settings_location = &args[1];
    }

    let settings = Config::builder()
        .add_source(config::File::with_name(settings_location))
        .build()
        .expect("Issue building configuration.")
        .try_deserialize()
        .expect("Issue parsing configuration.");

    clean_channel::execute(settings)
        .await
        .expect("Error cleaning.");
}
