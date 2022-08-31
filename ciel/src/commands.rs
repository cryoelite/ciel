
use paris::{error};

use serenity::{

    client::bridge::gateway::ShardId,
    framework::{
        standard::{

            macros::command, CommandResult, 
        },

    },

    model::prelude::Message,
    prelude::Context,

};


use crate::configuration::ShardManagerContainer;




#[command]
#[bucket = "General"]
pub async fn about(ctx: &Context, msg: &Message) -> CommandResult {
    msg.channel_id.say(&ctx.http, "Ciel!)").await?;

    Ok(())
}

#[command]
#[bucket = "General"]
pub async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    let data = ctx.data.read().await;
    let shard_manager = match data.get::<ShardManagerContainer>() {
        Some(v) => v,
        None => {
            error!("Unable to retrieve shard manager");
            msg.reply(ctx, "Internal error occured").await?;

            return Ok(());
        }
    };

    let manager = shard_manager.lock().await;
    let runners = manager.runners.lock().await;
    let runner = match runners.get(&ShardId(ctx.shard_id)) {
        Some(runner) => runner,
        None => {
            error!("Shard not found");
            msg.reply(ctx, "Internal error occured").await?;

            return Ok(());
        }
    };

    msg.reply(ctx, &format!("Pong! \nIt took {:?} ms", runner.latency))
        .await?;

    Ok(())
}

#[command]
#[bucket = "General"]
pub async fn flip_image(ctx: &Context, msg: &Message) -> CommandResult {
    //let flip_result = CielImage::flip(&msg.body.as_bytes()).await;
    //TODO: Port C++ Image matrix transformations and/or re-import CielImage crate
    msg.reply(ctx, &format!("Image flipper broken :/"))
        .await?;

    Ok(())
}