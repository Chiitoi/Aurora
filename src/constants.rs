use lazy_static::lazy_static;
use std::env;
use twilight_gateway::{EventTypeFlags, Intents};
use twilight_model::id::{Id, marker::{ApplicationMarker, GuildMarker}};

lazy_static! {
    pub static ref APPLICATION_ID: Id<ApplicationMarker> = Id::new(env::var("APPLICATION_ID").unwrap().parse::<u64>().unwrap());
    pub static ref BOT_TOKEN: String = env::var("BOT_TOKEN").unwrap();
    pub static ref DATABASE_URL: String = env::var("DATABASE_URL").unwrap();
    pub static ref DEVELOPMENT_GUILD_ID: Id<GuildMarker> = Id::new(env::var("DEVELOPMENT_GUILD_ID").unwrap().parse::<u64>().unwrap());
    pub static ref ENVIRONMENT: String = env::var("ENVIRONMENT").unwrap();
    pub static ref EVENT_TYPES: EventTypeFlags = EventTypeFlags::SHARD_PAYLOAD;
    pub static ref INTENTS: Intents = Intents::GUILDS | Intents::GUILD_MEMBERS | Intents::GUILD_MESSAGES | Intents::MESSAGE_CONTENT;
}