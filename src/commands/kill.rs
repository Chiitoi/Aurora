use crate::util::{helper::create_interaction_response, context::Context};
use rand::Rng;
use std::sync::Arc;
use super::Action;
use twilight_util::builder::{embed::EmbedBuilder, InteractionResponseDataBuilder};
use twilight_interactions::command::{CommandModel, CreateCommand, ResolvedUser};
use twilight_model::{application::interaction::ApplicationCommand, http::interaction::{InteractionResponse, InteractionResponseType}, user::User};

#[derive(CommandModel, CreateCommand)]
#[command(
    desc = "Fight someone",
    name = "kill"
)]
pub struct KillCommand {
    #[command(desc = "Your target")]
    target: ResolvedUser
}

impl KillCommand {
    pub async fn run(command: ApplicationCommand, context: &Arc<Context>) -> Result<InteractionResponse, anyhow::Error> {
        let guild_id = command.guild_id.unwrap();
        let member_id = command.author_id().unwrap();
        let member_name = command.member.unwrap().user.unwrap().name;
        let KillCommand { target } = KillCommand::from_interaction(command.data.into())?;
        let User { id: target_id, name: target_name, .. } = target.resolved;

        if member_id.eq(&target_id) {
            create_interaction_response(":heart: You changed your mind.", false)
        } else {
            let strings = vec![
                (format!(":axe: *You drop your axe mid-swing, **{target_name}** looks at you in shame.*"), None, None, 0xE9CA00),
                (format!(":axe: *You swing your axe at **{target_name}** slicing them in half.*"), Some(member_id), Some(target_id), 0x2FE900),
                (format!(":bow_and_arrow: *You aim your bow, but **{target_name}** wounds you first!*"), Some(target_id), Some(member_id), 0xDE4343),
                (format!(":bow_and_arrow: *You shoot at **{target_name}** piercing them with an arrow!*"), Some(member_id), Some(target_id), 0x2FE900),
                (format!(":boxing_glove: *You challenge **{target_name}** still lose...*"), Some(target_id), Some(member_id), 0xDE4343),
                (format!(":dagger: *You brought a knife to a bowfight, but **{target_name}** takes you out.*"), Some(target_id), Some(member_id), 0xDE4343),
                (format!(":dagger: *You stab **{target_name}** in the back of the heart.*"), Some(member_id), Some(target_id), 0x2FE900),
                (format!(":knife: *You lunge at **{target_name}** but clearly misjudged the distance.*"), None, None, 0xE9CA00),
            ];
            let index = rand::thread_rng().gen_range(0..strings.len());
            let result = strings.into_iter().nth(index).unwrap();
    
            if result.1.is_some() && result.2.is_some() {
                context.database().upsert_action(guild_id, result.1.unwrap(), result.2.unwrap(), Action::Kill.as_str()).await;
            }

            let (member_kill_count, target_kill_count) = context.database().read_kill_counts(guild_id, member_id, target_id).await;
            let title = format!("({member_kill_count}) {member_name} :crossed_swords: {target_name} ({target_kill_count})");
            let embed = EmbedBuilder::new()
                .color(result.3)
                .description(result.0)
                .title(title)
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
        }
    }
}