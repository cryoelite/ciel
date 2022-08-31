
use paris::{error, info, warn};
use serenity::{
    async_trait,
    client::bridge::gateway::{ShardManager},
    framework::{
        standard::{

            macros::{hook, group}, CommandResult, DispatchError,
        },

    },
    model::prelude::{Message, Ready},
    prelude::{Context, EventHandler,  TypeMapKey},

};
use tokio::sync::Mutex;
use std::{sync::Arc, pin::Pin};

use crate::url_worker::*;
use crate::commands::*;


pub static BUCKET_GENERAL: &str = "General";
pub static BUCKET_SETTINGS: &str = "Settings";

pub struct CielHandler;
#[async_trait]
impl EventHandler for CielHandler {
    async fn ready(&self, _: Context, ready: Ready) {
        info!("{} is connected and ready!", ready.user.name);
    }
}

pub struct ShardManagerContainer;

impl TypeMapKey for ShardManagerContainer {
    type Value = Arc<Mutex<ShardManager>>;
}

pub struct CommandHistory;

impl TypeMapKey for CommandHistory {
    type Value = Vec<String>;
}


#[hook]
pub async fn before(ctx: &Context, msg: &Message, command_name: &str) -> bool {
    info!("Captured {} from {}", &command_name, &msg.author.name);

    let mut data = ctx.data.write().await;
    let command_history = data
        .get_mut::<CommandHistory>()
        .expect("Expected CommandHistory in TypeMap.");

    if command_history.len() > 10 {
        command_history.clear();
    }

    command_history.push(format!("{} used {} command", msg.author.name, command_name));

    true
}

#[hook]
pub async fn after(_ctx: &Context, _msg: &Message, command_name: &str, command_result: CommandResult) {
    if let Ok(_) = command_result {
        info!("Processed command {}", &command_name);
    } else if let Err(why) = command_result {
        error!("Command {} has encountered an err: {}", &command_name, &why);
    }
}

#[hook]
pub async fn unknown_command(_ctx: &Context, _msg: &Message, unknown_command_name: &str) {
    warn!("Unable to find command {}", unknown_command_name);
}

#[hook]
pub async fn dispatch_error(ctx: &Context, msg: &Message, error: DispatchError, _command_name: &str) {
    if let DispatchError::Ratelimited(info) = error {
        if info.is_first_try {
            let _ = msg
                .channel_id
                .say(
                    &ctx.http,
                    &format!("Try this again in {} seconds.", info.as_secs()),
                )
                .await;
        }
    }
}
#[hook]
pub async fn normal_message(ctx: &Context, msg: &Message) {
    info!("Processing normal message");
    let check_result = check_url(&msg.content);
    if let Some(result) = check_result {
        let mut cleaned_urls = String::new();
        for elem in result {
            let clean_url = clean_url(&elem[..]).await;
            if let Ok(url) = &clean_url {
                cleaned_urls.push_str(&url);
                cleaned_urls.push('\n');
            }
        }
        let reply = msg.reply(ctx, &cleaned_urls).await;
        match reply {
            Ok(_) => info!("Successfully checked and replied clean URLs"),
            Err(why) => error!("Failed reply as {}", why),
        }
    }
}

#[group]
#[commands(ping, flip_image, about)]
struct General;

#[group]
#[prefix("set")]
struct Settings;

pub fn check_admin<'fut>(
    _ctx: &'fut Context,
    _msg: &'fut Message,
) -> Pin<Box<(dyn std::future::Future<Output = bool> + std::marker::Send + 'fut)>> {
    let result = Box::pin(async { true });
    //TODO: Extract and setup user roles
    return result;
}