use chrono::Utc;
use crate::{database::Ship, util::{context::Context, helper::{create_interaction_response, humanize}}};
use std::sync::Arc;
use twilight_interactions::command::{CommandModel, CreateCommand, ResolvedUser};
use twilight_model::{
    application::component::{action_row::ActionRow, button::Button},
    application::{interaction::ApplicationCommand, component::{button::ButtonStyle, Component}},
    http::interaction::{InteractionResponse, InteractionResponseType}
};
use twilight_util::builder::{embed::{EmbedBuilder, EmbedFieldBuilder}, InteractionResponseDataBuilder};

#[derive(CommandModel, CreateCommand)]
#[command(
    desc = "Manage your ship",
    name = "ship"
)]
pub enum ShipCommand {
    #[command(name = "create")]
    Create(ShipCreate),
    #[command(name = "rename")]
    Rename(ShipRename),
    #[command(name = "show")]
    Show(ShipShow),
    #[command(name = "sink")]
    Sink(ShipSink)

}

#[derive(CommandModel, CreateCommand)]
#[command(desc = "Creates a ship with that special someone", name = "create")]
pub struct ShipCreate {
    #[command(desc = "The special someone")]
    user: ResolvedUser
}

#[derive(CommandModel, CreateCommand)]
#[command(desc = "Rename your ship", name = "rename")]
pub struct ShipRename {
    #[command(desc = "Your new ship name")]
    name: String
}

#[derive(CommandModel, CreateCommand)]
#[command(desc = "Sinks your current ship", name = "sink")]
pub struct ShipSink {}

#[derive(CommandModel, CreateCommand)]
#[command(desc = "Shows your ship", name = "show")]
pub struct ShipShow {}

impl ShipCommand {
    pub async fn run(command: ApplicationCommand, context: &Arc<Context>) -> Result<InteractionResponse, anyhow::Error>  {
        let guild_id = command.guild_id.unwrap();
        let member_id = command.author_id().unwrap();
        let member_name = command.member.unwrap().user.unwrap().name;
        let options = ShipCommand::from_interaction(command.data.into())?;

        match options {
            ShipCommand::Create(ShipCreate { user }) => {
                let resolved_user_id = user.resolved.id;

                if let Some(_) =  context.database().read_ship(guild_id, member_id).await {
                    create_interaction_response("You are already shipped!", true)
                } else if let Some(_) =  context.database().read_ship(guild_id, resolved_user_id).await {
                    create_interaction_response("That person is already shipped!", true)
                } else {
                    let components = vec![
                        Component::ActionRow(ActionRow {
                            components: vec![
                                Component::Button(Button {
                                    custom_id: Some("accept".into()),
                                    disabled: false,
                                    emoji: None,
                                    label: Some("Accept".into()),
                                    style: ButtonStyle::Primary,
                                    url: None
                                }),
                                Component::Button(Button {
                                    custom_id: Some("reject".into()),
                                    disabled: false,
                                    emoji: None,
                                    label: Some("Reject".into()),
                                    style: ButtonStyle::Primary,
                                    url: None
                                })
                            ]
                        })
                    ];
                    let content = format!("Hey <@{}>! It looks like **{}** wants to get it on... 😏 The choice is yours!", user.resolved.id, member_name);
    
                    Ok(
                        InteractionResponse {
                            data: Some(
                                InteractionResponseDataBuilder::new()
                                    .components(components)
                                    .content(content)
                                    .build()
                            ),
                            kind: InteractionResponseType::ChannelMessageWithSource
                        }
                    )
                }              
            },
            ShipCommand::Rename(ShipRename { name }) => match context.database().read_ship(guild_id, member_id).await {
                Some(_) => {
                    context.database().update_ship(guild_id, member_id, name).await;
                    create_interaction_response("You have updated your ship name!", true)
                },
                None => create_interaction_response("You are not shipped!", true)
            },
            ShipCommand::Show(_) => match context.database().read_ship(guild_id, member_id).await {
                Some(Ship { id_one, id_two, name, created_at, .. }) => {
                    let counts = context.database().read_action_counts(guild_id, id_one, id_two).await;
                    let now = Utc::now().naive_utc();
                    let milliseconds = now.timestamp_millis() - created_at.timestamp_millis();
                    let duration = humanize(milliseconds as u64);
                    let embed = EmbedBuilder::new()
                        .color(0xF8F8FF)
                        .description(format!("<@{}> loves, and is loved by, <@{}>", id_one, id_two))
                        .field(EmbedFieldBuilder::new("Counts", counts.to_string()).build())
                        .field(EmbedFieldBuilder::new("Duration", duration).build())
                        .title(format!("The \"{name}\" ship"))
                        .build();

                    Ok(
                        InteractionResponse {
                            data: Some(
                                InteractionResponseDataBuilder::new()
                                    .embeds([embed])
                                    .build()
                            ),
                            kind: InteractionResponseType::ChannelMessageWithSource
                        }
                    )
                },
                None => create_interaction_response("You are not shipped!", true)
            },
            ShipCommand::Sink(_) => match context.database().read_ship(guild_id, member_id).await {
                Some(_) => {
                    context.database().delete_ship(guild_id, member_id).await;
                    create_interaction_response("You have sunk your ship!", true)
                },
                None => create_interaction_response("You are not shipped!", true)
            }
        }
    }
}