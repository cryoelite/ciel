
pub mod commands;
pub mod configuration;
pub mod url_worker;
pub mod models;

use std::{env, sync::Arc};

use paris::{error, info, warn};

use serenity::{
    http::Http,
    prelude::GatewayIntents,
    Client,
};
use serenity::framework::{
        standard::{
            buckets::LimitedFor,
        },
        StandardFramework,
    };


use crate::configuration::*;



#[tokio::main]
pub async fn start_bot() {
    let token = env::var("DISCORD_API_KEY").expect("Environment variable not set for API key");

    let http = Http::new(&token);

    let bot_id = match http.get_current_application_info().await {
        Ok(_) => match http.get_current_user().await {
            Ok(bot_id) => bot_id.id,
            Err(why) => panic!("Could not access the bot id: {:?}", why),
        },
        Err(why) => panic!("Could not access application info: {:?}", why),
    };

    let framework = StandardFramework::new()
        .configure(|c| {
            c.with_whitespace(true)
                .on_mention(Some(bot_id))
                .prefix(".")
                .delimiters(vec![", ", ","])
        })
        .before(before)
        .after(after)
        .unrecognised_command(unknown_command)
        .on_dispatch_error(dispatch_error)
        .normal_message(normal_message)
        .bucket(BUCKET_GENERAL, |b| {
            b.delay(1).limit_for(LimitedFor::Channel)
        })
        .await
        .bucket(BUCKET_SETTINGS, |b| b.check(check_admin))
        .await
        .group(&GENERAL_GROUP)
        .group(&SETTINGS_GROUP);

    let intents = GatewayIntents::all();
    let mut client = Client::builder(&token, intents)
        .event_handler(CielHandler)
        .framework(framework)
        .type_map_insert::<CommandHistory>(Vec::new())
        .await
        .expect("Err creating client");

    {
        let mut data = client.data.write().await;
        data.insert::<ShardManagerContainer>(Arc::clone(&client.shard_manager));
    }

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}

