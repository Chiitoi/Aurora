use crate::{
    commands::*,
    constants::DEVELOPMENT_GUILD_ID,
    util::context::Context
};
use std::sync::Arc;
use twilight_model::application::command::Command;

pub async fn register_guild_commands(context: &Arc<Context>) {

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
    ];
    
    context
        .interaction_client()
        .set_guild_commands(*DEVELOPMENT_GUILD_ID, &commands)
        .exec()
        .await
        .unwrap();
}