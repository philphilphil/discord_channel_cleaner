# discord_channel_cleaner
Simple software to delete messages in discord channels after a certain amount of time.
Its possible to keep images and only delete text messages.

## Installation
via cargo:
```bash
cargo install discord_channel_cleaner
```
via docker: 
```
tbd
```
## Usage
### Start
Simply call `discord_channel_cleaner`.
If the `settings.toml` is not in the cwd, the path can be given as an argument: `discord_channel_cleaner /home/phil/chan_clean_conf.toml`.

### Setup
You need to register a Discord application with a bot.
The required gateway intents are "Read Messages/View Channels", "Read Message History", "Send Messages" and "Manage Messages".

Copy (or create) the `settings.toml` file and replace with your values.

Each channel needs his own channel section. The ID's can be copied easily via right-click if you activate "Developer Mode" on your discord account.
