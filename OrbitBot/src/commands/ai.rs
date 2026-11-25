use crate::ai::client::AIClient;
use crate::ai::context::{add_message, clear as clear_conversation, get_or_create};
use crate::ai::usage::{check_cooldown, check_request_limit, record_usage};
use crate::kb::search_kb;
use crate::models::*;
use crate::state::{Context, Error};
use poise::serenity_prelude as serenity;
use serenity::CreateEmbed;

async fn admin_check(ctx: Context<'_>) -> Result<bool, Error> {
    let cache = ctx.serenity_context().cache.clone();
    let member = ctx.author_member().await;
    Ok(member.map_or(false, |m| {
        m.permissions(cache).map_or(false, |p| p.administrator())
    }))
}

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

#[poise::command(slash_command)]
pub async fn ask(
    ctx: Context<'_>,
    #[description = "Your question"] question: String,
) -> Result<(), Error> {
    ctx.defer().await?;

    let discord_id = ctx.author().id.get();

    let mut storage = ctx.data().storage.lock().await;
    let (config, provider) = match ai_config_and_provider(&storage) {
        Ok(c) => c,
        Err(e) => {
            drop(storage);
            ctx.send(
                poise::CreateReply::default()
                    .ephemeral(true)
                    .content(e.to_string()),
            )
            .await?;
            return Ok(());
        }
    };

    if let Some(remaining) = check_cooldown(&storage, discord_id, config.cooldown_secs) {
        drop(storage);
        ctx.send(
            poise::CreateReply::default()
                .ephemeral(true)
                .content(format!(
                    "Please wait {} seconds before your next AI request.",
                    remaining
                )),
        )
        .await?;
        return Ok(());
    }

    if check_request_limit(&storage, discord_id, config.per_user_request_limit) {
        drop(storage);
        ctx.send(
            poise::CreateReply::default()
                .ephemeral(true)
                .content("You have reached your per-user AI request limit."),
        )
        .await?;
        return Ok(());
    }

    let conversation = get_or_create(&mut storage, discord_id);
    let mut messages: Vec<AIMessage> = conversation.messages.clone();

    let kb_results = search_kb(&question, 3);
    if !kb_results.is_empty() {
        let kb_context: String = kb_results
            .iter()
            .map(|e| format!("[{}] {}", e.title, e.content))
            .collect::<Vec<_>>()
            .join("\n\n");
        messages.insert(0, AIMessage {
            role: "system".to_string(),
            content: format!(
                "다음은 CBGPU 지식 기반에서 참고할 정보입니다. 답변할 때 이 내용을 참고하세요:\n\n{}",
                kb_context
            ),
        });
    }

    messages.push(AIMessage {
        role: "user".to_string(),
        content: question.clone(),
    });

    let input_text: String = messages
        .iter()
        .map(|m| m.content.as_str())
        .collect::<Vec<_>>()
        .join(" ");
    let input_tokens = (input_text.len() / 4) as u32;
    if input_tokens > config.max_input_tokens as u32 {
        drop(storage);
        ctx.send(
            poise::CreateReply::default()
                .ephemeral(true)
                .content("Input exceeds maximum token limit."),
        )
        .await?;
        return Ok(());
    }

    add_message(
        &mut storage,
        discord_id,
        "user".to_string(),
        question.clone(),
        config.context_message_limit,
    );
    storage.save()?;
    drop(storage);

    let client = AIClient::new(provider.clone());
    let response = client
        .chat_completion(&messages, config.max_output_tokens)
        .await;

    let mut storage = ctx.data().storage.lock().await;
    match response {
        Ok(content) => {
            let output_tokens = (content.len() / 4) as u32;
            add_message(
                &mut storage,
                discord_id,
                "assistant".to_string(),
                content.clone(),
                config.context_message_limit,
            );
            record_usage(
                &mut storage,
                discord_id,
                provider.name.clone(),
                provider.model.clone(),
                input_tokens,
                output_tokens,
            );
            storage.save()?;
            drop(storage);

            let embed = CreateEmbed::new()
                .title("AI Response")
                .field("Question", question, false)
                .field("Answer", content, false)
                .field("Model", provider.model, true);
            ctx.send(poise::CreateReply::default().embed(embed)).await?;
        }
        Err(e) => {
            drop(storage);
            ctx.send(
                poise::CreateReply::default()
                    .ephemeral(true)
                    .content(format!("AI request failed: {}", e)),
            )
            .await?;
        }
    }

    Ok(())
}

#[poise::command(slash_command)]
pub async fn explain(
    ctx: Context<'_>,
    #[description = "Text to explain"] text: String,
) -> Result<(), Error> {
    ctx.defer().await?;

    let discord_id = ctx.author().id.get();

    let mut storage = ctx.data().storage.lock().await;
    let (config, provider) = match ai_config_and_provider(&storage) {
        Ok(c) => c,
        Err(e) => {
            drop(storage);
            ctx.send(
                poise::CreateReply::default()
                    .ephemeral(true)
                    .content(e.to_string()),
            )
            .await?;
            return Ok(());
        }
    };

    if let Some(remaining) = check_cooldown(&storage, discord_id, config.cooldown_secs) {
        drop(storage);
        ctx.send(
            poise::CreateReply::default()
                .ephemeral(true)
                .content(format!(
                    "Please wait {} seconds before your next AI request.",
                    remaining
                )),
        )
        .await?;
        return Ok(());
    }

    if check_request_limit(&storage, discord_id, config.per_user_request_limit) {
        drop(storage);
        ctx.send(
            poise::CreateReply::default()
                .ephemeral(true)
                .content("You have reached your per-user AI request limit."),
        )
        .await?;
        return Ok(());
    }

    let conversation = get_or_create(&mut storage, discord_id);
    let mut messages: Vec<AIMessage> = conversation.messages.clone();
    let prompt = format!("Explain the following text:\n\n{}", text);
    messages.push(AIMessage {
        role: "user".to_string(),
        content: prompt.clone(),
    });

    let input_text: String = messages
        .iter()
        .map(|m| m.content.as_str())
        .collect::<Vec<_>>()
        .join(" ");
    let input_tokens = (input_text.len() / 4) as u32;
    if input_tokens > config.max_input_tokens as u32 {
        drop(storage);
        ctx.send(
            poise::CreateReply::default()
                .ephemeral(true)
                .content("Input exceeds maximum token limit."),
        )
        .await?;
        return Ok(());
    }

    add_message(
        &mut storage,
        discord_id,
        "user".to_string(),
        prompt,
        config.context_message_limit,
    );
    storage.save()?;
    drop(storage);

    let client = AIClient::new(provider.clone());
    let response = client
        .chat_completion(&messages, config.max_output_tokens)
        .await;

    let mut storage = ctx.data().storage.lock().await;
    match response {
        Ok(content) => {
            let output_tokens = (content.len() / 4) as u32;
            add_message(
                &mut storage,
                discord_id,
                "assistant".to_string(),
                content.clone(),
                config.context_message_limit,
            );
            record_usage(
                &mut storage,
                discord_id,
                provider.name.clone(),
                provider.model.clone(),
                input_tokens,
                output_tokens,
            );
            storage.save()?;
            drop(storage);

            let embed = CreateEmbed::new()
                .title("Explanation")
                .field("Text", text, false)
                .field("Explanation", content, false)
                .field("Model", provider.model, true);
            ctx.send(poise::CreateReply::default().embed(embed)).await?;
        }
        Err(e) => {
            drop(storage);
            ctx.send(
                poise::CreateReply::default()
                    .ephemeral(true)
                    .content(format!("AI request failed: {}", e)),
            )
            .await?;
        }
    }

    Ok(())
}

#[poise::command(slash_command)]
pub async fn summarize(
    ctx: Context<'_>,
    #[description = "Text to summarize"] text: String,
) -> Result<(), Error> {
    ctx.defer().await?;

    let discord_id = ctx.author().id.get();

    let mut storage = ctx.data().storage.lock().await;
    let (config, provider) = match ai_config_and_provider(&storage) {
        Ok(c) => c,
        Err(e) => {
            drop(storage);
            ctx.send(
                poise::CreateReply::default()
                    .ephemeral(true)
                    .content(e.to_string()),
            )
            .await?;
            return Ok(());
        }
    };

    if let Some(remaining) = check_cooldown(&storage, discord_id, config.cooldown_secs) {
        drop(storage);
        ctx.send(
            poise::CreateReply::default()
                .ephemeral(true)
                .content(format!(
                    "Please wait {} seconds before your next AI request.",
                    remaining
                )),
        )
        .await?;
        return Ok(());
    }

    if check_request_limit(&storage, discord_id, config.per_user_request_limit) {
        drop(storage);
        ctx.send(
            poise::CreateReply::default()
                .ephemeral(true)
                .content("You have reached your per-user AI request limit."),
        )
        .await?;
        return Ok(());
    }

    let conversation = get_or_create(&mut storage, discord_id);
    let mut messages: Vec<AIMessage> = conversation.messages.clone();
    let prompt = format!("Summarize the following text:\n\n{}", text);
    messages.push(AIMessage {
        role: "user".to_string(),
        content: prompt.clone(),
    });

    let input_text: String = messages
        .iter()
        .map(|m| m.content.as_str())
        .collect::<Vec<_>>()
        .join(" ");
    let input_tokens = (input_text.len() / 4) as u32;
    if input_tokens > config.max_input_tokens as u32 {
        drop(storage);
        ctx.send(
            poise::CreateReply::default()
                .ephemeral(true)
                .content("Input exceeds maximum token limit."),
        )
        .await?;
        return Ok(());
    }

    add_message(
        &mut storage,
        discord_id,
        "user".to_string(),
        prompt,
        config.context_message_limit,
    );
    storage.save()?;
    drop(storage);

    let client = AIClient::new(provider.clone());
    let response = client
        .chat_completion(&messages, config.max_output_tokens)
        .await;

    let mut storage = ctx.data().storage.lock().await;
    match response {
        Ok(content) => {
            let output_tokens = (content.len() / 4) as u32;
            add_message(
                &mut storage,
                discord_id,
                "assistant".to_string(),
                content.clone(),
                config.context_message_limit,
            );
            record_usage(
                &mut storage,
                discord_id,
                provider.name.clone(),
                provider.model.clone(),
                input_tokens,
                output_tokens,
            );
            storage.save()?;
            drop(storage);

            let embed = CreateEmbed::new()
                .title("Summary")
                .field("Summary", content, false)
                .field("Model", provider.model, true);
            ctx.send(poise::CreateReply::default().embed(embed)).await?;
        }
        Err(e) => {
            drop(storage);
            ctx.send(
                poise::CreateReply::default()
                    .ephemeral(true)
                    .content(format!("AI request failed: {}", e)),
            )
            .await?;
        }
    }

    Ok(())
}

#[poise::command(slash_command)]
pub async fn translate(
    ctx: Context<'_>,
    #[description = "Text to translate"] text: String,
    #[description = "Target language"] target_language: String,
) -> Result<(), Error> {
    ctx.defer().await?;

    let discord_id = ctx.author().id.get();

    let mut storage = ctx.data().storage.lock().await;
    let (config, provider) = match ai_config_and_provider(&storage) {
        Ok(c) => c,
        Err(e) => {
            drop(storage);
            ctx.send(
                poise::CreateReply::default()
                    .ephemeral(true)
                    .content(e.to_string()),
            )
            .await?;
            return Ok(());
        }
    };

    if let Some(remaining) = check_cooldown(&storage, discord_id, config.cooldown_secs) {
        drop(storage);
        ctx.send(
            poise::CreateReply::default()
                .ephemeral(true)
                .content(format!(
                    "Please wait {} seconds before your next AI request.",
                    remaining
                )),
        )
        .await?;
        return Ok(());
    }

    if check_request_limit(&storage, discord_id, config.per_user_request_limit) {
        drop(storage);
        ctx.send(
            poise::CreateReply::default()
                .ephemeral(true)
                .content("You have reached your per-user AI request limit."),
        )
        .await?;
        return Ok(());
    }

    let conversation = get_or_create(&mut storage, discord_id);
    let mut messages: Vec<AIMessage> = conversation.messages.clone();
    let prompt = format!(
        "Translate the following text to {}:\n\n{}",
        target_language, text
    );
    messages.push(AIMessage {
        role: "user".to_string(),
        content: prompt.clone(),
    });

    let input_text: String = messages
        .iter()
        .map(|m| m.content.as_str())
        .collect::<Vec<_>>()
        .join(" ");
    let input_tokens = (input_text.len() / 4) as u32;
    if input_tokens > config.max_input_tokens as u32 {
        drop(storage);
        ctx.send(
            poise::CreateReply::default()
                .ephemeral(true)
                .content("Input exceeds maximum token limit."),
        )
        .await?;
        return Ok(());
    }

    add_message(
        &mut storage,
        discord_id,
        "user".to_string(),
        prompt,
        config.context_message_limit,
    );
    storage.save()?;
    drop(storage);

    let client = AIClient::new(provider.clone());
    let response = client
        .chat_completion(&messages, config.max_output_tokens)
        .await;

    let mut storage = ctx.data().storage.lock().await;
    match response {
        Ok(content) => {
            let output_tokens = (content.len() / 4) as u32;
            add_message(
                &mut storage,
                discord_id,
                "assistant".to_string(),
                content.clone(),
                config.context_message_limit,
            );
            record_usage(
                &mut storage,
                discord_id,
                provider.name.clone(),
                provider.model.clone(),
                input_tokens,
                output_tokens,
            );
            storage.save()?;
            drop(storage);

            let embed = CreateEmbed::new()
                .title(format!("Translation to {}", target_language))
                .field("Original", text, false)
                .field("Translation", content, false)
                .field("Model", provider.model, true);
            ctx.send(poise::CreateReply::default().embed(embed)).await?;
        }
        Err(e) => {
            drop(storage);
            ctx.send(
                poise::CreateReply::default()
                    .ephemeral(true)
                    .content(format!("AI request failed: {}", e)),
            )
            .await?;
        }
    }

    Ok(())
}

#[poise::command(slash_command)]
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

#[poise::command(slash_command)]
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

#[poise::command(slash_command)]
pub async fn ai_usage(
    ctx: Context<'_>,
    #[description = "User to view"] user: Option<serenity::UserId>,
) -> Result<(), Error> {
    let target = user.unwrap_or_else(|| ctx.author().id);
    let discord_id = target.get();

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
        .title(format!("AI Usage — <@{}>", discord_id))
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

#[poise::command(slash_command)]
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
        config.default_provider = Some(p);
    }
    if let Some(m) = model {
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
