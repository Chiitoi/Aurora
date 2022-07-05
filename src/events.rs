use crate::{commands::{action::Action, get_interaction_response}, util::context::Context};
use std::sync::Arc;
use twilight_gateway::Event;
use twilight_model::{
    application::interaction::{Interaction, ApplicationCommand},
    channel::message::MessageFlags,
    http::interaction::{InteractionResponse, InteractionResponseType},
};
use twilight_util::builder::{embed::EmbedBuilder, InteractionResponseDataBuilder};

pub async fn handle(event: Event, context: Arc<Context>) {    
    context.cache().update(&event);

    match event {
        Event::GuildCreate(guild) => context.database().create_setting(guild.id).await,
        Event::GuildDelete(guild) => context.database().delete_setting(guild.id).await,
        Event::InteractionCreate(interaction) => {
            if let Interaction::ApplicationCommand(command) = interaction.0 {
                let ApplicationCommand { token, id, .. } = *command.clone();
                let mut interaction_response = match command.data.name.as_str() {
                    "bite" => get_interaction_response(*command, &context, Action::Bite).await,
                    "cuddle" => get_interaction_response(*command, &context, Action::Cuddle).await,
                    "handhold" => get_interaction_response(*command, &context, Action::Handhold).await,
                    "kiss" => get_interaction_response(*command, &context, Action::Kiss).await,
                    "pat" => get_interaction_response(*command, &context, Action::Pat).await,
                    "pinch" => get_interaction_response(*command, &context, Action::Pinch).await,
                    "poke" => get_interaction_response(*command, &context, Action::Poke).await,
                    "punch" => get_interaction_response(*command, &context, Action::Punch).await,
                    "tickle" => get_interaction_response(*command, &context, Action::Tickle).await,
                    name => {
                        let embed = EmbedBuilder::new()
                            .color(0xFF0000)
                            .description(format!("Received unknown command \"{name}\""))
                            .build();

                        Ok(
                            InteractionResponse {
                                data: Some(
                                    InteractionResponseDataBuilder::new()
                                    .embeds([embed])
                                    .flags(MessageFlags::EPHEMERAL)
                                    .build()
                                ),
                                kind: InteractionResponseType::ChannelMessageWithSource
                            }
                        )
                    }
                };

                if interaction_response.is_err() {
                    let embed = EmbedBuilder::new()
                        .color(0xFF0000)
                        .description("Unable to process command")
                        .build();

                    interaction_response = Ok(
                        InteractionResponse {
                            data: Some(
                                InteractionResponseDataBuilder::new()
                                .embeds([embed])
                                .flags(MessageFlags::EPHEMERAL)
                                .build()
                            ),
                            kind: InteractionResponseType::ChannelMessageWithSource
                        }
                    )   
                }

                if let Err(_) = context
                    .interaction_client()
                    .create_response(id, &token, &interaction_response.unwrap())
                    .exec()
                    .await 
                {
                    println!("Something went wrong!")    
                }
            }
        },
        Event::Ready(ready) => println!("{}#{} is online!", ready.user.name, ready.user.discriminator),
        _ => {}
    }

}