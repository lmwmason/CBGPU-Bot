use crate::commands::mission::admin_check;
use crate::models::RegistrationRecord;
use crate::reminders::{fetch_members, send_reminders};
use crate::state::{Context, Error};
use crate::util::truncate;
use chrono::Utc;
use poise::serenity_prelude as serenity;
use serenity::CreateEmbed;

async fn require_admin(ctx: Context<'_>) -> Result<bool, Error> {
    if admin_check(ctx).await? {
        return Ok(true);
    }
    ctx.send(
        poise::CreateReply::default()
            .ephemeral(true)
            .content("Only administrators can use this command."),
    )
    .await?;
    Ok(false)
}

fn guild_of(ctx: Context<'_>) -> Option<serenity::GuildId> {
    ctx.guild_id()
}

/// Mark a member as having registered their school on CBGPU.
#[poise::command(slash_command, guild_only, rename = "cbgpu_mark_registered")]
pub async fn mark_registered(
    ctx: Context<'_>,
    #[description = "Member who registered"] user: serenity::User,
    #[description = "School they registered"] school: Option<String>,
) -> Result<(), Error> {
    if !require_admin(ctx).await? {
        return Ok(());
    }
    let actor = ctx.author().id.get();
    let id = user.id.get();

    let mut storage = ctx.data().storage.lock().await;
    let existing = storage
        .storage()
        .registrations
        .iter()
        .position(|r| r.discord_id == id);
    let updated = existing.is_some();
    match existing {
        Some(i) => {
            let r = &mut storage.storage_mut().registrations[i];
            if school.is_some() {
                r.school = school.clone();
            }
            r.marked_by = actor;
            r.marked_at = Utc::now();
        }
        None => storage.storage_mut().registrations.push(RegistrationRecord {
            discord_id: id,
            school: school.clone(),
            marked_by: actor,
            marked_at: Utc::now(),
        }),
    }
    storage.audit_log(
        actor,
        "mark_registered".to_string(),
        id.to_string(),
        format!("school = {}", school.as_deref().unwrap_or("-")),
    );
    storage.save()?;
    drop(storage);

    ctx.send(poise::CreateReply::default().ephemeral(true).content(format!(
        "<@{id}> is {} as registered{}. They will no longer get the 23:00 reminder.",
        if updated { "updated" } else { "marked" },
        school.map(|s| format!(" ({s})")).unwrap_or_default()
    )))
    .await?;
    Ok(())
}

/// Remove a member's registered mark (they will get reminders again).
#[poise::command(slash_command, guild_only, rename = "cbgpu_unmark_registered")]
pub async fn unmark_registered(
    ctx: Context<'_>,
    #[description = "Member"] user: serenity::User,
) -> Result<(), Error> {
    if !require_admin(ctx).await? {
        return Ok(());
    }
    let actor = ctx.author().id.get();
    let id = user.id.get();
    let mut storage = ctx.data().storage.lock().await;
    let before = storage.storage().registrations.len();
    storage.storage_mut().registrations.retain(|r| r.discord_id != id);
    let removed = storage.storage().registrations.len() != before;
    if removed {
        storage.audit_log(
            actor,
            "unmark_registered".to_string(),
            id.to_string(),
            "removed registered mark".to_string(),
        );
        storage.save()?;
    }
    drop(storage);

    ctx.send(poise::CreateReply::default().ephemeral(true).content(if removed {
        format!("<@{id}> is no longer marked as registered.")
    } else {
        format!("<@{id}> was not marked as registered.")
    }))
    .await?;
    Ok(())
}

/// Show who has registered and who has not.
#[poise::command(slash_command, guild_only, rename = "cbgpu_registration_status")]
pub async fn registration_status(ctx: Context<'_>) -> Result<(), Error> {
    if !require_admin(ctx).await? {
        return Ok(());
    }
    let Some(guild_id) = guild_of(ctx) else { return Ok(()) };
    ctx.defer_ephemeral().await?;

    let members = fetch_members(ctx.http(), guild_id).await?;
    let storage = ctx.data().storage.lock().await;
    let records = storage.storage().registrations.clone();
    drop(storage);

    let mut done = String::new();
    let mut missing = String::new();
    let (mut done_n, mut missing_n) = (0, 0);
    for m in &members {
        match records.iter().find(|r| r.discord_id == m.user.id.get()) {
            Some(r) => {
                done_n += 1;
                done.push_str(&format!(
                    "• <@{}>{}\n",
                    m.user.id.get(),
                    r.school.as_ref().map(|s| format!(" — {s}")).unwrap_or_default()
                ));
            }
            None => {
                missing_n += 1;
                missing.push_str(&format!("• <@{}>\n", m.user.id.get()));
            }
        }
    }

    let embed = CreateEmbed::new()
        .title("School registration status")
        .field(
            format!("✅ Registered ({done_n})"),
            truncate(if done.is_empty() { "—" } else { &done }, 1000),
            false,
        )
        .field(
            format!("❌ Not registered ({missing_n})"),
            truncate(if missing.is_empty() { "—" } else { &missing }, 1000),
            false,
        )
        .footer(serenity::CreateEmbedFooter::new(
            "Unregistered members get a DM every day at 23:00 KST.",
        ));
    ctx.send(poise::CreateReply::default().ephemeral(true).embed(embed))
        .await?;
    Ok(())
}

/// Send the reminder DM to all unregistered members right now.
#[poise::command(slash_command, guild_only, rename = "cbgpu_remind_unregistered")]
pub async fn remind_unregistered(ctx: Context<'_>) -> Result<(), Error> {
    if !require_admin(ctx).await? {
        return Ok(());
    }
    let Some(guild_id) = guild_of(ctx) else { return Ok(()) };
    ctx.defer_ephemeral().await?;

    let report = send_reminders(ctx.http(), guild_id, &ctx.data().storage).await?;
    let mut msg = format!(
        "Reminders sent: {}. Already registered: {}. DMs failed: {}.",
        report.sent,
        report.registered,
        report.failed.len()
    );
    if !report.failed.is_empty() {
        msg.push_str(&format!("\nCould not DM: {}", report.failed.join(", ")));
    }
    ctx.send(
        poise::CreateReply::default()
            .ephemeral(true)
            .content(truncate(&msg, 1900)),
    )
    .await?;
    Ok(())
}
