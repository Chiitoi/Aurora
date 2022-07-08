use crate::util::{context::Context, helper::{handle_command, handle_component}};
use std::sync::Arc;
use twilight_gateway::Event;
use twilight_model::application::interaction::Interaction;

pub async fn handle(event: Event, context: Arc<Context>) {    
    context.cache().update(&event);

    match event {
        Event::GuildCreate(guild) => context.database().create_setting(guild.id).await,
        Event::GuildDelete(guild) => context.database().delete_setting(guild.id).await,
        Event::InteractionCreate(interaction) => match interaction.0 {
            Interaction::ApplicationCommand(command) => handle_command(*command, context).await,
            Interaction::MessageComponent(component) => handle_component(*component, context).await,
            _ => {},
        },
        Event::Ready(ready) => println!("{}#{} is online!", ready.user.name, ready.user.discriminator),
        _ => {}
    }

}