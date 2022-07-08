use crate::util::helper::create_interaction_response;
use rand::Rng;
use twilight_interactions::command::{CommandModel, CreateCommand};
use twilight_model::{application::interaction::ApplicationCommand, http::interaction::InteractionResponse};

#[derive(CommandModel, CreateCommand)]
#[command(
    desc = "Rate something",
    name = "rate"
)]
pub struct RateCommand {
    #[command(desc = "What to rate")]
    query: String
}

impl RateCommand {
    pub async fn run(command: ApplicationCommand) -> Result<InteractionResponse, anyhow::Error> {
        let RateCommand { query } = RateCommand::from_interaction(command.data.into())?;
        let rating = rand::thread_rng().gen_range(0..10);
        let rating_text = format!(":thinking: Hmm.. I rate **{query}** a {rating}/10! :heart:");

        create_interaction_response(&rating_text, false)
    }
}