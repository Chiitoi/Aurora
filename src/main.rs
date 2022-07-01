mod constants;
mod database;
mod events;
mod util;

use constants::{
    BOT_TOKEN,
    EVENT_TYPES,
    INTENTS
};
use dotenv::dotenv;
use futures_util::StreamExt;
use serde::de::DeserializeSeed;
use std::sync::Arc;
use twilight_gateway::{Cluster, Event};
use twilight_model::gateway::event::{gateway::GatewayEventDeserializer, GatewayEvent};
use util::context::Context;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();
    tracing_subscriber::fmt::init();

    let (cluster, mut events) = Cluster::builder(BOT_TOKEN.to_string(), *INTENTS)
        .event_types(*EVENT_TYPES)
        .build()
        .await?;
    let context = Arc::new(Context::new(cluster));
    let context_clone = context.clone();

    tokio::spawn(async move {
        context_clone.cluster.up().await;
        context_clone.database.create_tables().await;
    });

    while let Some((_shard_id, event)) = events.next().await {
        if let Event::ShardPayload(payload) = event {
            let json_str = String::from_utf8(payload.bytes).unwrap();
            let event_deserializer = GatewayEventDeserializer::from_json(&json_str).unwrap();
            let mut json_deserializer = serde_json::Deserializer::from_str(&json_str);
            let gateway_event = event_deserializer.deserialize(&mut json_deserializer)?;

            if let GatewayEvent::Dispatch(_event_id, dispatch_event) = gateway_event {
                tokio::spawn(events::handle(*dispatch_event, context.clone()));
            }
        }
    }

    Ok(())
}