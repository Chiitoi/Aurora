use lazy_static::lazy_static;
use std::env;
use twilight_gateway::{EventTypeFlags, Intents};

lazy_static! {
    pub static ref BOT_TOKEN: String = env::var("BOT_TOKEN").unwrap();
    pub static ref EVENT_TYPES: EventTypeFlags = EventTypeFlags::SHARD_PAYLOAD;
    pub static ref INTENTS: Intents = Intents::GUILDS | Intents::GUILD_MEMBERS | Intents::GUILD_MESSAGES | Intents::MESSAGE_CONTENT;
}