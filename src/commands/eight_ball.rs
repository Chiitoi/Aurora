use crate::util::helper::create_interaction_response;
use rand::{thread_rng, seq::SliceRandom};
use twilight_interactions::command::{CommandModel, CreateCommand};
use twilight_model::{application::interaction::ApplicationCommand, http::interaction::InteractionResponse};

#[derive(CommandModel, CreateCommand)]
#[command(
    desc = "Ask Aurora something",
    name = "8ball"
)]
pub struct EightBallCommand {
    #[command(desc = "Your question")]
    question: String
}

impl EightBallCommand {
    pub async fn run(command: ApplicationCommand) -> Result<InteractionResponse, anyhow::Error> {
        let EightBallCommand { question } = EightBallCommand::from_interaction(command.data.into())?;
        let answers = vec![
            "As I see it, yes.",
            "Ask again later.",
            "Better not tell you now.",
            "Cannot predict now.",
            "Concentrate and ask again.",
            "Don't count on it.",
            "It is certain.",
            "It is decidedly so.",
            "Most likely.",
            "My reply is no.",
            "My sources say no.",
            "Outlook good.",
            "Outlook not so good.",
            "Reply hazy, try again.",
            "Signs point to yes.",
            "Very doubtful.",
            "Without a doubt.",
            "Yes, definitely.",
            "Yes.",
            "You may rely on it."
        ];
        let mut rng = thread_rng();
        let answer = answers.choose(&mut rng).unwrap();
        let answer_text = format!("**Q:** {question}\n**A:** {answer}");

        create_interaction_response(&answer_text, false)
    }
}