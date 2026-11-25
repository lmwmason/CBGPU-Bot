use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::prelude::*;
use std::env;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.author.bot {
            return;
        }
        if msg.content == "!ping" {
            let _ = msg.channel_id.say(&ctx.http, "Pong!").await;
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} connected!", ready.user.name);
    }
}

#[tokio::main]
async fn main() {
    let token = env::var("DISCORD_TOKEN").expect("DISCORD_TOKEN is required.");
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    let mut client = Client::builder(&token, intents)
        .event_handler(Handler)
        .await
        .expect("Failed to create Discord client.");

    if let Err(why) = client.start().await {
        let detail = format!("{why:?}");
        eprintln!("Client error: {detail}");
        if detail.contains("DisallowedGatewayIntents") {
            eprintln!("Open your application's Bot settings in the Discord Developer Portal.");
            eprintln!("Enable and save Message Content Intent under Privileged Gateway Intents, then restart the bot.");
        }
    }
}
