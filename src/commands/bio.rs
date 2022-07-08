use crate::util::{context::Context, helper::create_interaction_response};
use std::sync::Arc;
use twilight_interactions::command::{CommandModel, CreateCommand};
use twilight_model::{application::interaction::ApplicationCommand, http::interaction::{InteractionResponse, InteractionResponseType}};
use twilight_util::builder::InteractionResponseDataBuilder;

#[derive(CommandModel, CreateCommand)]
#[command(
    desc = "Manage your bio",
    name = "bio"
)]
pub enum BioCommand {
    #[command(name = "clear")]
    Clear(BioClear),
    #[command(name = "set")]
    Set(BioSet),
    #[command(name = "show")]
    Show(BioShow)
}

#[derive(CommandModel, CreateCommand)]
#[command(desc = "Clears your bio", name = "clear")]
pub struct BioClear {}

#[derive(CommandModel, CreateCommand)]
#[command(desc = "Sets your bio", name = "set")]
pub struct BioSet {
    #[command(desc = "Your new bio")]
    bio: String
}

#[derive(CommandModel, CreateCommand)]
#[command(desc = "Shows your bio", name = "show")]
pub struct BioShow {}

impl BioCommand {
    pub async fn run(command: ApplicationCommand, context: &Arc<Context>) -> Result<InteractionResponse, anyhow::Error> {
        let guild_id = command.guild_id.unwrap();
        let member_id = command.author_id().unwrap();
        let options = BioCommand::from_interaction(command.data.into())?;

        match options {
            BioCommand::Clear(_) => {
                context.database().update_bio(guild_id, member_id, None).await;
                create_interaction_response("Bio cleared!", true)
            },
            BioCommand::Set(BioSet { bio }) => {
                if bio.chars().count() > 250 {
                    create_interaction_response("Bio must be fewer than 250 characters!", true)
                } else {
                    context.database().update_bio(guild_id, member_id, Some(bio)).await;
                    create_interaction_response("Bio set!", true)
                }
            },
            BioCommand::Show(_) => {
                match context.database().read_bio(guild_id, member_id).await {
                    Some(bio) => Ok(
                        InteractionResponse {
                            data: Some(
                                InteractionResponseDataBuilder::new()
                                    .content(format!("```{bio}```"))
                                    .build()
                            ),
                            kind: InteractionResponseType::ChannelMessageWithSource
                        }
                    ),
                    None => create_interaction_response("No bio set...", true)
                }
            }
        }
    }
}