use crate::util::context::Context;
use std::sync::Arc;
use twilight_gateway::Event;
use twilight_model::gateway::event::DispatchEvent;

pub async fn handle(dispatch_event: DispatchEvent, context: Arc<Context>) {
    match dispatch_event {
        DispatchEvent::ChannelDelete(_) => todo!(),
        DispatchEvent::GuildCreate(_) => todo!(),
        DispatchEvent::GuildDelete(_) => todo!(),
        DispatchEvent::InteractionCreate(_) => todo!(),
        DispatchEvent::MessageCreate(_) => todo!(),
        DispatchEvent::Ready(ready) => println!("{}#{} is online!", ready.user.name, ready.user.discriminator),
        DispatchEvent::RoleDelete(_) => todo!(),
        DispatchEvent::VoiceStateUpdate(_) => todo!(),
        _ => {
            context.cache.update(&Event::from(dispatch_event));
        }
    }
}