mod commands;
mod constants;
mod database;
mod events;
mod util;

use constants::{BOT_TOKEN, INTENTS};
use dotenv::dotenv;
use futures_util::StreamExt;
use twilight_http::client::Client;
use std::sync::Arc;
use twilight_gateway::Cluster;
use util::context::Context;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();
    tracing_subscriber::fmt::init();

    let http_client = Client::new(BOT_TOKEN.to_string());
    let (cluster, mut events) = Cluster::builder(BOT_TOKEN.to_string(), *INTENTS)
        .build()
        .await?;
    let context = Arc::new(Context::new(http_client, cluster));
    let context_clone = context.clone();

    context_clone.database().create_tables().await;
    util::helper::register_commands(&context).await;

    tokio::spawn(async move {
        context_clone.cluster().up().await;    
    });

    while let Some((_shard_id, event)) = events.next().await {
        tokio::spawn(events::handle(event, context.clone()));
    }

    Ok(())
}