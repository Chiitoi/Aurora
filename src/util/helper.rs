use crate::{
    commands::*,
    constants::{DEVELOPMENT_GUILD_ID, ENVIRONMENT},
    util::context::Context
};
use std::sync::Arc;
use twilight_interactions::command::CreateCommand;
use twilight_model::{
    application::{command::Command, interaction::{ApplicationCommand, message_component::MessageComponentInteraction}},
    channel::message::MessageFlags,
    http::interaction::{InteractionResponse, InteractionResponseType}   
};
use twilight_util::builder::{embed::EmbedBuilder, InteractionResponseDataBuilder};

pub fn create_interaction_response(description: &str, ephemeral: bool) -> Result<InteractionResponse, anyhow::Error> {
    let embed = EmbedBuilder::new()
        .color(0xF8F8FF)
        .description(description)
        .build();
    let mut interaction_response = InteractionResponseDataBuilder::new()
        .embeds([embed]);

    interaction_response = if ephemeral {
        interaction_response.flags(MessageFlags::EPHEMERAL)
    } else {
        interaction_response
    };

    Ok(
        InteractionResponse {
            data: Some(interaction_response.build()),
            kind: InteractionResponseType::ChannelMessageWithSource
        }
    )
}

pub async fn handle_command(command: ApplicationCommand, context: Arc<Context>) {
    let ApplicationCommand { id, token, ..  } = command.clone();
    let mut interaction_response = match command.data.name.as_str() {
        "bio" => BioCommand::run(command, &context).await,
        "bite" => get_interaction_response(command, &context, Action::Bite).await,
        "cuddle" => get_interaction_response(command, &context, Action::Cuddle).await,
        "handhold" => get_interaction_response(command, &context, Action::Handhold).await,
        "kill" => KillCommand::run(command, &context).await,
        "kiss" => get_interaction_response(command, &context, Action::Kiss).await,
        "pat" => get_interaction_response(command, &context, Action::Pat).await,
        "pinch" => get_interaction_response(command, &context, Action::Pinch).await,
        "poke" => get_interaction_response(command, &context, Action::Poke).await,
        "rate" => RateCommand::run(command).await,
        "ship" => ShipCommand::run(command, &context).await,
        "punch" => get_interaction_response(command, &context, Action::Punch).await,
        "tickle" => get_interaction_response(command, &context, Action::Tickle).await,
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

    context
        .interaction_client()
        .create_response(id, &token, &interaction_response.unwrap())
        .exec()
        .await
        .unwrap();   
}

pub async fn handle_component(component: MessageComponentInteraction, context: Arc<Context>) {
    let mut button_presser_id = None;

    let interaction_response = if let Some(button_presser) = component.member {
        if let Some(mention) = component.message.mentions.into_iter().nth(0) {
            if button_presser.user.unwrap().id.eq(&mention.id) {
                if component.data.custom_id == "accept" {
                    button_presser_id = Some(mention.id);
                    create_interaction_response(":tada: Congrats! Your ship has sailed! :tada:", false)
                } else {
                    create_interaction_response(format!("**{}** has sank the ship, it looks like it was never meant to be :pensive:", mention.name).as_str(), false)
                }
            } else {
                create_interaction_response("This ship was not intended for you.", true)
            }
        } else {
            let embed = EmbedBuilder::new()
                .color(0xFF0000)
                .description("Unable to process ship mentions")
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
    } else {
        let embed = EmbedBuilder::new()
            .color(0xFF0000)
            .description("Unable to process component interaction")
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
    };


    if let Ok(_) = context
        .interaction_client()
        .create_response(component.id, &component.token, &interaction_response.unwrap())
        .exec()
        .await
    {
        if let Some(id) = button_presser_id {
            let guild_id = component.guild_id.unwrap();
            let initiator_id = component.message.interaction.unwrap().user.id;

            context.database().create_ship(guild_id, initiator_id, id).await;
        }
    }  
}

pub fn humanize(mut milliseconds: u64) -> String {
    let days = milliseconds / 86_400_000;
    milliseconds = milliseconds % 86_400_000;
    let hours = milliseconds / 3_600_000;
    milliseconds = milliseconds % 3_600_000;
    let minutes = milliseconds / 60_000;
    milliseconds = milliseconds % 60_000;
    let seconds = milliseconds / 1_000;


    let parts = vec![(days, "d"), (hours, "h"), (minutes, "m"), (seconds, "s")];
    let duration: String = parts.iter().filter_map(|(value, unit)| match *unit {
           _ if *value > 0 => Some(format!("{value}{unit}")),
           _ => None
    }).collect::<Vec<String>>().join(" ");

    duration
}

pub async fn register_commands(context: &Arc<Context>) {
    let commands: Vec<Command> = vec![
        Action::create_action_command(Action::Bite, "30% chance to flinch the target".into()),
        Action::create_action_command(Action::Cuddle, "Big spoon or little spoon?".into()),
        Action::create_action_command(Action::Handhold, "In case your hand gets lonely...".into()),
        Action::create_action_command(Action::Kiss, "ALL THE PDA!!!".into()),
        Action::create_action_command(Action::Pat, ":3".into()),
        Action::create_action_command(Action::Pinch, "Grab those other cheeks ;)".into()),
        Action::create_action_command(Action::Poke, "👉".into()),
        Action::create_action_command(Action::Punch, "For when someone needs to be knocked out".into()),
        Action::create_action_command(Action::Tickle, "You know what this is...".into()),
        BioCommand::create_command().into(),
        KillCommand::create_command().into(),
        RateCommand::create_command().into(),
        ShipCommand::create_command().into()
    ];
    let interaction_client = context.interaction_client();

    if ENVIRONMENT.to_lowercase() == "production" {
        interaction_client
            .set_global_commands(&commands)
            .exec()
            .await
            .unwrap();
    } else {
        interaction_client
            .set_guild_commands(*DEVELOPMENT_GUILD_ID, &commands)
            .exec()
            .await
            .unwrap();
    }
}