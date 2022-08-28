use std::{env};

use serenity::{framework::{StandardFramework, standard::{CommandResult, macros::{command, group}}}, prelude::{GatewayIntents, EventHandler, Context}, Client, async_trait, model::prelude::Message};

struct Handler;

#[async_trait]
impl EventHandler for Handler {}

#[group]
#[commands(ping)]
struct General;

#[tokio::main]
async fn main() {
    let serenity = StandardFramework::new().configure(|c| c.prefix("!")).group(&GENERAL_GROUP);

    let token = env::var("BOT_TOKEN").expect("BOT_TOKEN not defined");
    let intents = GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT;

    let mut client = Client::builder(token, intents).event_handler(Handler).framework(serenity).await.expect("Error creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}

#[command]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, "`Running on rust!`").await?;
    Ok(())
}
