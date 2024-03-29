# discord_channel_cleaner
Simple software to delete messages in discord channels after a certain amount of time.
It was created for photography gallery channels to delete text messages that are x hours old but keep images.

If `keep_images` is set to true, messages with attachments (images) and links from the configured allow list are not deleted.
If false, all messages get deleted.

## Installation
via cargo:
```bash
cargo install discord_channel_cleaner
```
via docker: 
```bash
docker pull phib/discord_channel_cleaner
```
## Usage
### Setup
You need to register a Discord application with a bot.
The required gateway intents are "Read Messages/View Channels", "Read Message History", "Send Messages" and "Manage Messages".

Copy (or create) the `settings.toml` file and replace with your values.

Each channel needs his own channel section. The ID's can be copied easily via right-click if you activate "Developer Mode" on your discord account.
### Start
Simply call `discord_channel_cleaner`.
If the `settings.toml` is not in the cwd, the path can be given as an argument: `discord_channel_cleaner /home/phil/chan_clean_conf.toml`.
#### With docker
Via docker run, mount local `settings.toml` to `/dcc/settings.toml`.
```bash
docker run -it --mount type=bind,source="$(pwd)"/settings.toml,target=/dcc/settings.toml,readonly discord_channel_cleaner
```
