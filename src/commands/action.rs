use crate::util::context::Context;
use serde::Deserialize;
use std::{str, sync::Arc};
use hyper::{body::Buf, Uri};
use twilight_model::{
    application::command::{Command, CommandType},
    application::interaction::{ApplicationCommand, application_command::CommandOptionValue},
    http::interaction::{InteractionResponse, InteractionResponseType}
};
use twilight_util::builder::{
    command::{CommandBuilder, UserBuilder},
    embed::{EmbedBuilder, ImageSource, EmbedFooterBuilder},
    InteractionResponseDataBuilder
};

pub enum Action {
    Bite,
    Cuddle,
    Handhold,
    Hug,
    Kill,
    Kiss,
    Pat,
    Pinch,
    Poke,
    Punch,
    Tickle
}

impl Action {
    pub fn action_phrase(&self) -> &'static str {
        match self {
            Action::Bite => "bites",
            Action::Cuddle => "cuddles",
            Action::Handhold => "holds hands with",
            Action::Hug => "hugs",
            Action::Kill => "kills",
            Action::Kiss => "kisses",
            Action::Pat => "pats",
            Action::Pinch => "pinches",
            Action::Poke => "pokes",
            Action::Punch => "punches",
            Action::Tickle => "tickles"
        }
    }

    pub fn as_plural(&self) -> &'static str {
        match self {
            Action::Bite => "bites",
            Action::Cuddle => "cuddles",
            Action::Handhold => "handholds",
            Action::Hug => "hugs",
            Action::Kill => "kills",
            Action::Kiss => "kisses",
            Action::Pat => "pats",
            Action::Pinch => "pinches",
            Action::Poke => "pokes",
            Action::Punch => "punches",
            Action::Tickle => "tickles"
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Action::Bite => "bite",
            Action::Cuddle => "cuddle",
            Action::Handhold => "handhold",
            Action::Hug => "hug",
            Action::Kill => "kill",
            Action::Kiss => "kiss",
            Action::Pat => "pat",
            Action::Pinch => "pinch",
            Action::Poke => "poke",
            Action::Punch => "punch",
            Action::Tickle => "tickle"
        }
    }

    pub fn create_action_command(action: Action, description: String) -> Command {
        CommandBuilder::new(action.as_str().into(), description, CommandType::ChatInput)
            .option(UserBuilder::new("user".into(), "The recipient".into()))
            .build()   
    }
}

#[derive(Deserialize, Debug)]
struct OtakuGIFResponse {
    url: String
}

async fn act_and_count(command: ApplicationCommand, context: &Arc<Context>, action: &Action) -> (Option<String>, String, String) {
    let guild_id = command.guild_id.unwrap();
    let member_id = command.author_id().unwrap();
    let recipient_id = match command.data.options.first() {
        Some(option) => match option.value {
            CommandOptionValue::User(id) => id,
            _ => member_id
        }
        None => member_id,
    };
    let count = context.database().upsert_action(guild_id, member_id, recipient_id, action.as_str()).await;
    let content = if member_id.eq(&recipient_id) {
        None
    } else {
        Some(format!("<@{recipient_id}>"))
    };
    let description = if member_id.eq(&recipient_id) {
        format!("*<@{member_id}> {} themselves!*", action.as_plural())
    } else {
        format!("*<@{member_id}> {} you!*", action.action_phrase())
    };
    let footer_text = match count {
        1 =>  if member_id.eq(&recipient_id) {
            format!("That's your first {} from yourself!", action.as_str())
        } else {
            format!("That's their first {} from you!", action.as_str())
        },
        _ => format!("That's {count} {} now!", action.as_plural())
    };

    (content, description, footer_text)
}

async fn get_image_source(context: &Arc<Context>, action: &Action) -> ImageSource {
    let formatted_uri = format!("https://api.otakugifs.xyz/gif?reaction={}", action.as_str());
    let uri = formatted_uri.parse::<Uri>().unwrap();
    let res = context.hyper().get(uri).await.unwrap();
    let body = hyper::body::aggregate(res).await.unwrap();
    let OtakuGIFResponse { url } = serde_json::from_reader(body.reader()).unwrap();

    ImageSource::url(url).unwrap()
}

pub async fn get_interaction_response(command: ApplicationCommand, context: &Arc<Context>, action: Action) -> Result<InteractionResponse, anyhow::Error> {
    let (content, description, footer_text) = act_and_count(command, &context, &action).await;
    let image_source = get_image_source(context, &action).await;
    let embed = EmbedBuilder::new()
        .color(0xF8F8FF)
        .description(description)
        .footer(EmbedFooterBuilder::new(footer_text))
        .image(image_source)
        .build();
    let interation_response_data = match content {
        Some(content) => InteractionResponseDataBuilder::new()
            .content(content)
            .embeds([embed])
            .build(),
        None => InteractionResponseDataBuilder::new()
            .embeds([embed])
            .build()
    };

    Ok(InteractionResponse { data: Some(interation_response_data), kind: InteractionResponseType::ChannelMessageWithSource })
}