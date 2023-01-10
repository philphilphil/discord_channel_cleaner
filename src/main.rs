mod clean_channel;
mod settings;
mod utilities;

use config::Config;
use settings::Settings;
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

#[tokio::main]
async fn main() {
    // Log/Output settings
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    info!("Loading configuration..");

    let mut config = Config::builder()
        .add_source(config::File::with_name("settings.toml"))
        .build()
        .expect("Issue building configuration.");

    let settings: Settings = config.try_deserialize().unwrap();

    clean_channel::execute(settings).await;

    info!("asd");
}
