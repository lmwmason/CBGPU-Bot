use crate::ai::client::AIClient;
use crate::ai::context::{add_message, clear as clear_conversation};
use crate::ai::usage::{check_cooldown, record_usage};
use crate::kb::search_kb;
use crate::commands::mission::admin_check;
use crate::models::*;
use crate::util::{estimate_tokens, truncate, EMBED_DESC_MAX, EMBED_FIELD_MAX};
use crate::state::{Context, Error};
use poise::serenity_prelude as serenity;
use serenity::CreateEmbed;

fn ai_config_and_provider(
    storage: &crate::storage::Store,
) -> Result<(AIConfig, AIProviderConfig), Error> {
    let config = storage
        .storage()
        .ai_config
        .clone()
        .ok_or("AI is not configured.")?;
    if !config.enabled {
        return Err("AI is not enabled.".into());
    }
    let provider_name = config
        .default_provider
        .clone()
        .ok_or("No default provider set.")?;
    let provider = config
        .providers
        .iter()
        .find(|p| p.name == provider_name)
        .ok_or("Default provider not found.")?
        .clone();
    Ok((config, provider))
}

const SYSTEM_PROMPT: &str = "You are CBGPU-Bot, the assistant of the CBGPU ambassador Discord server. \
CBGPU (https://cbgpu.com) is a Korean platform that connects schools and projects: school life, projects, AI/research tools, GPU computing with Jupyter, public projects and contests. \
Answer in the user's language, concisely. If you are not sure about a CBGPU-specific detail, say so instead of guessing.";

async fn reply_ephemeral(ctx: Context<'_>, msg: impl Into<String>) -> Result<(), Error> {
    ctx.send(
        poise::CreateReply::default()
            .ephemeral(true)
            .content(msg.into()),
    )
    .await?;
    Ok(())
}

/// Shared implementation for /ask, /explain, /summarize and /translate.
/// `prompt` is what the model sees; `shown_input` (label, text) is echoed in the embed.
async fn run_ai(
    ctx: Context<'_>,
    prompt: String,
    use_kb: bool,
    title: String,
    shown_input: Option<(&str, String)>,
) -> Result<(), Error> {
    let discord_id = ctx.author().id.get();

    let Some(_in_flight) = ctx.data().try_begin_ai(discord_id) else {
        return reply_ephemeral(ctx, "Your previous AI request is still running. Please wait for it to finish.").await;
    };

    let (config, provider, messages, input_tokens) = {
        let storage = ctx.data().storage.lock().await;
        let (config, provider) = match ai_config_and_provider(&storage) {
            Ok(c) => c,
            Err(e) => {
                drop(storage);
                return reply_ephemeral(ctx, e.to_string()).await;
            }
        };
        if let Some(remaining) = check_cooldown(&storage, discord_id, config.cooldown_secs) {
            drop(storage);
            return reply_ephemeral(
                ctx,
                format!("Please wait {remaining} seconds before your next AI request."),
            )
            .await;
        }

        let mut system = SYSTEM_PROMPT.to_string();
        if use_kb {
            let kb = search_kb(&prompt, 3);
            if !kb.is_empty() {
                system.push_str("\n\n다음은 CBGPU 지식 기반에서 참고할 정보입니다. 답변할 때 이 내용을 참고하세요:\n\n");
                system.push_str(
                    &kb.iter()
                        .map(|e| format!("[{}] {}", e.title, e.content))
                        .collect::<Vec<_>>()
                        .join("\n\n"),
                );
            }
        }

        let mut messages = vec![AIMessage {
            role: "system".to_string(),
            content: system,
        }];
        if let Some(idx) = storage.find_ai_conversation(discord_id) {
            messages.extend(storage.storage().ai_conversations[idx].messages.clone());
        }
        messages.push(AIMessage {
            role: "user".to_string(),
            content: prompt.clone(),
        });

        let input_tokens: usize = messages.iter().map(|m| estimate_tokens(&m.content)).sum();
        if input_tokens > config.max_input_tokens {
            drop(storage);
            return reply_ephemeral(
                ctx,
                "Input exceeds maximum token limit. Try a shorter text or run /cbgpu_clear.",
            )
            .await;
        }
        (config, provider, messages, input_tokens as u32)
    };

    ctx.defer().await?;

    let response = AIClient::new(provider.clone())
        .chat_completion(&messages, config.max_output_tokens)
        .await;

    match response {
        Ok(content) => {
            {
                let mut storage = ctx.data().storage.lock().await;
                let limit = config.context_message_limit;
                add_message(&mut storage, discord_id, "user".into(), prompt, limit);
                add_message(&mut storage, discord_id, "assistant".into(), content.clone(), limit);
                record_usage(
                    &mut storage,
                    discord_id,
                    provider.name.clone(),
                    provider.model.clone(),
                    input_tokens,
                    estimate_tokens(&content) as u32,
                );
                storage.save()?;
            }

            let mut embed = CreateEmbed::new()
                .title(title)
                .description(truncate(&content, EMBED_DESC_MAX))
                .footer(serenity::CreateEmbedFooter::new(format!(
                    "Model: {}",
                    provider.model
                )));
            if let Some((label, text)) = shown_input {
                embed = embed.field(label, truncate(&text, EMBED_FIELD_MAX), false);
            }
            ctx.send(poise::CreateReply::default().embed(embed)).await?;
        }
        Err(e) => {
            ctx.send(
                poise::CreateReply::default()
                    .ephemeral(true)
                    .content(truncate(&format!("AI request failed: {e}"), 1900)),
            )
            .await?;
        }
    }
    Ok(())
}

#[poise::command(slash_command, rename = "cbgpu_ask")]
pub async fn ask(
    ctx: Context<'_>,
    #[description = "Your question"] question: String,
) -> Result<(), Error> {
    run_ai(
        ctx,
        question.clone(),
        true,
        "AI Response".into(),
        Some(("Question", question)),
    )
    .await
}

#[poise::command(slash_command, rename = "cbgpu_explain")]
pub async fn explain(
    ctx: Context<'_>,
    #[description = "Text to explain"] text: String,
) -> Result<(), Error> {
    let prompt = format!("Explain the following text:\n\n{text}");
    run_ai(ctx, prompt, false, "Explanation".into(), Some(("Text", text))).await
}

#[poise::command(slash_command, rename = "cbgpu_summarize")]
pub async fn summarize(
    ctx: Context<'_>,
    #[description = "Text to summarize"] text: String,
) -> Result<(), Error> {
    let prompt = format!("Summarize the following text:\n\n{text}");
    run_ai(ctx, prompt, false, "Summary".into(), Some(("Original", text))).await
}

#[poise::command(slash_command, rename = "cbgpu_translate")]
pub async fn translate(
    ctx: Context<'_>,
    #[description = "Text to translate"] text: String,
    #[description = "Target language"] target_language: String,
) -> Result<(), Error> {
    let prompt = format!("Translate the following text to {target_language}. Output only the translation:\n\n{text}");
    run_ai(
        ctx,
        prompt,
        false,
        format!("Translation to {target_language}"),
        Some(("Original", text)),
    )
    .await
}

#[poise::command(slash_command, rename = "cbgpu_model")]
pub async fn model(ctx: Context<'_>) -> Result<(), Error> {
    let storage = ctx.data().storage.lock().await;
    let config = match storage.storage().ai_config.clone() {
        Some(c) => c,
        None => {
            drop(storage);
            ctx.send(
                poise::CreateReply::default()
                    .ephemeral(true)
                    .content("AI is not configured."),
            )
            .await?;
            return Ok(());
        }
    };

    let default_provider = config.default_provider.as_deref().unwrap_or("None");
    let default_model = config.default_model.as_deref().unwrap_or("None");

    let mut description = format!(
        "Default provider: {}\nDefault model: {}\n\nProviders:\n",
        default_provider, default_model
    );
    for p in &config.providers {
        description.push_str(&format!("• {} — {} ({})\n", p.name, p.model, p.base_url));
    }
    drop(storage);

    let embed = CreateEmbed::new()
        .title("AI Model Configuration")
        .description(description);
    ctx.send(poise::CreateReply::default().embed(embed)).await?;
    Ok(())
}

#[poise::command(slash_command, rename = "cbgpu_clear")]
pub async fn clear(ctx: Context<'_>) -> Result<(), Error> {
    let discord_id = ctx.author().id.get();

    let mut storage = ctx.data().storage.lock().await;
    let removed = clear_conversation(&mut storage, discord_id);
    storage.save()?;
    drop(storage);

    if removed {
        ctx.send(
            poise::CreateReply::default()
                .ephemeral(true)
                .content("Your conversation history has been cleared."),
        )
        .await?;
    } else {
        ctx.send(
            poise::CreateReply::default()
                .ephemeral(true)
                .content("No conversation history to clear."),
        )
        .await?;
    }
    Ok(())
}

#[poise::command(slash_command, rename = "cbgpu_ai_usage")]
pub async fn ai_usage(
    ctx: Context<'_>,
    #[description = "User to view"] user: Option<serenity::UserId>,
) -> Result<(), Error> {
    let target = user.unwrap_or_else(|| ctx.author().id);
    let discord_id = target.get();

    if target != ctx.author().id && !admin_check(ctx).await? {
        return reply_ephemeral(ctx, "Only administrators can view other users' AI usage.").await;
    }

    let storage = ctx.data().storage.lock().await;
    let record = match storage.find_ai_usage(discord_id) {
        Some(idx) => storage.storage().ai_usage[idx].clone(),
        None => {
            drop(storage);
            ctx.send(
                poise::CreateReply::default()
                    .ephemeral(true)
                    .content("No AI usage records found."),
            )
            .await?;
            return Ok(());
        }
    };
    drop(storage);

    let embed = CreateEmbed::new()
        .title("AI Usage")
        .description(format!("<@{}>", discord_id))
        .field("Provider", record.provider, true)
        .field("Model", record.model, true)
        .field("Requests", record.request_count.to_string(), true)
        .field("Input Tokens", record.input_tokens.to_string(), true)
        .field("Output Tokens", record.output_tokens.to_string(), true)
        .field(
            "Last Request",
            record.last_request.map_or("Never".to_string(), |t| {
                t.format("%Y-%m-%d %H:%M:%S").to_string()
            }),
            true,
        );

    ctx.send(poise::CreateReply::default().embed(embed)).await?;
    Ok(())
}

#[poise::command(slash_command, rename = "cbgpu_ai_config")]
pub async fn ai_config(
    ctx: Context<'_>,
    #[description = "Enable or disable AI"] enabled: Option<bool>,
    #[description = "Default provider name"] provider: Option<String>,
    #[description = "Default model name"] model: Option<String>,
) -> Result<(), Error> {
    let discord_id = ctx.author().id.get();

    if !admin_check(ctx).await? {
        ctx.send(
            poise::CreateReply::default()
                .ephemeral(true)
                .content("Only administrators can configure AI."),
        )
        .await?;
        return Ok(());
    }

    let mut storage = ctx.data().storage.lock().await;
    let config = match storage.storage_mut().ai_config.as_mut() {
        Some(c) => c,
        None => {
            drop(storage);
            ctx.send(
                poise::CreateReply::default()
                    .ephemeral(true)
                    .content("AI is not configured. Add a provider first."),
            )
            .await?;
            return Ok(());
        }
    };

    if let Some(e) = enabled {
        config.enabled = e;
    }
    if let Some(p) = provider {
        if !config.providers.iter().any(|x| x.name == p) {
            drop(storage);
            return reply_ephemeral(ctx, format!("Provider \"{p}\" does not exist.")).await;
        }
        config.default_provider = Some(p);
    }
    if let Some(m) = model {
        // The model is stored on the provider, which is what requests actually use.
        let default = config.default_provider.clone();
        if let Some(p) = config.providers.iter_mut().find(|p| Some(&p.name) == default.as_ref()) {
            p.model = m.clone();
        }
        config.default_model = Some(m);
    }

    storage.audit_log(
        discord_id,
        "ai_config".to_string(),
        "ai".to_string(),
        "Updated AI configuration".to_string(),
    );
    storage.save()?;
    drop(storage);

    ctx.send(
        poise::CreateReply::default()
            .ephemeral(true)
            .content("AI configuration updated."),
    )
    .await?;
    Ok(())
}
