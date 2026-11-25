mod ai;
mod commands;
mod kb;
mod models;
mod reminders;
mod school_sync;
mod state;
mod util;
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
    let intents = serenity::GatewayIntents::non_privileged() | serenity::GatewayIntents::GUILD_MEMBERS;

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![
                commands::general::info(),
                commands::ambassador::register(),
                commands::ambassador::ambassador_approve(),
                commands::ambassador::ambassador_reject(),
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
                commands::registration::mark_registered(),
                commands::registration::unmark_registered(),
                commands::registration::registration_status(),
                commands::registration::remind_unregistered(),
                commands::cbgpu::school_participation(),
                commands::cbgpu::features(),
                commands::cbgpu::schools(),
                commands::cbgpu::links(),
            ],
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                // Real CBGPU server: nightly registration reminders run here.
                let guild_id = serenity::GuildId::new(1572221094952173738);
                // Test server: slash commands only.
                let test_guild_id = serenity::GuildId::new(1575070127102230622);
                // Slash commands are registered in every guild listed here.
                let command_guilds = [guild_id, test_guild_id];
                for id in command_guilds {
                    if let Err(error) =
                        poise::builtins::register_in_guild(ctx, &framework.options().commands, id)
                            .await
                    {
                        eprintln!("Failed to register commands in guild {id}: {error}");
                    }
                }
                println!("Bot is ready.");

                let mut store = Store::load()?;
                if store.ensure_ai_config_from_env() {
                    println!("AI config created from environment.");
                }
                store.save()?;
                if std::env::var("AI_API_KEY").is_err() {
                    eprintln!("Warning: AI_API_KEY is not set; AI commands will fail.");
                }
                let storage = Arc::new(Mutex::new(store));
                tokio::spawn(school_sync::sync_loop(storage.clone()));
                tokio::spawn(reminders::reminder_loop(
                    ctx.http.clone(),
                    guild_id,
                    storage.clone(),
                ));
                Ok(Data {
                    storage,
                    ai_in_flight: Default::default(),
                })
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
