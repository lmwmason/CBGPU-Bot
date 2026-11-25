mod ai;
mod commands;
mod kb;
mod models;
mod state;
mod storage;

use poise::serenity_prelude as serenity;
use state::Data;
use std::env;
use std::sync::Arc;
use storage::Store;
use tokio::sync::Mutex;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    let token = env::var("DISCORD_TOKEN").expect("DISCORD_TOKEN is not set.");
    let intents = serenity::GatewayIntents::non_privileged();

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![
                commands::general::info(),
                commands::general::add(),
                commands::ambassador::register(),
                commands::ambassador::profile(),
                commands::ambassador::school(),
                commands::ambassador::members(),
                commands::ambassador::schools(),
                commands::ambassador::team(),
                commands::ambassador::activity(),
                commands::ambassador::leaderboard(),
                commands::mission::mission_create(),
                commands::mission::mission_edit(),
                commands::mission::mission_close(),
                commands::mission::mission_delete(),
                commands::mission::missions(),
                commands::mission::mission(),
                commands::mission::mission_participate(),
                commands::mission::mission_submit(),
                commands::mission::submission_approve(),
                commands::mission::submission_reject(),
                commands::mission::school_leaderboard(),
                commands::ai::ask(),
                commands::ai::explain(),
                commands::ai::summarize(),
                commands::ai::translate(),
                commands::ai::model(),
                commands::ai::clear(),
                commands::ai::ai_usage(),
                commands::ai::ai_config(),
                commands::cbgpu::cbgpu(),
                commands::cbgpu::features(),
                commands::cbgpu::schools(),
                commands::cbgpu::links(),
            ],
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                let guild_id = serenity::GuildId::new(1575070127102230622);
                poise::builtins::register_in_guild(ctx, &framework.options().commands, guild_id)
                    .await?;
                println!("Bot is ready.");

                let storage = Arc::new(Mutex::new(Store::load()?));
                Ok(Data { storage })
            })
        })
        .build();

    let mut client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .await
        .expect("Failed to create the Discord client.");

    if let Err(error) = client.start().await {
        eprintln!("Client error: {error:?}");
    }
}
