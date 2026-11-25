use crate::models::{ActivityRecord, Ambassador, AmbassadorStatus, School};
use crate::state::{Context, Error};
use poise::serenity_prelude as serenity;
use serenity::CreateEmbed;

#[poise::command(slash_command)]
pub async fn register(
    ctx: Context<'_>,
    #[description = "Your full name"] name: String,
    #[description = "Your school name"] school: String,
    #[description = "Your grade"] grade: Option<String>,
    #[description = "Introduction"] introduction: String,
    #[description = "Reason for applying"] reason: String,
) -> Result<(), Error> {
    let discord_id = ctx.author().id.get();

    let mut storage = ctx.data().storage.lock().await;

    if storage.find_ambassador_by_discord(discord_id).is_some() {
        drop(storage);
        ctx.send(
            poise::CreateReply::default()
                .ephemeral(true)
                .content("You are already registered as an ambassador."),
        )
        .await?;
        return Ok(());
    }

    if storage.find_school_by_name(&school).is_none() {
        storage
            .storage_mut()
            .schools
            .push(School::new(school.clone()));
    }

    let ambassador = Ambassador::new(discord_id, name, school, grade, introduction, reason);
    storage.storage_mut().ambassadors.push(ambassador);
    storage.audit_log(
        discord_id,
        "register".to_string(),
        "ambassador".to_string(),
        "Registered as ambassador".to_string(),
    );
    storage.save()?;
    drop(storage);

    ctx.send(poise::CreateReply::default().ephemeral(true).content(
        "You have been registered as a pending ambassador. An admin will review your application.",
    ))
    .await?;
    Ok(())
}

#[poise::command(slash_command)]
pub async fn profile(
    ctx: Context<'_>,
    #[description = "User to view"] user: Option<serenity::UserId>,
) -> Result<(), Error> {
    let target = user.unwrap_or_else(|| ctx.author().id);
    let discord_id = target.get();

    let storage = ctx.data().storage.lock().await;
    let index = storage.find_ambassador_by_discord(discord_id);
    drop(storage);

    match index {
        Some(idx) => {
            let storage = ctx.data().storage.lock().await;
            let a = storage.storage().ambassadors[idx].clone();
            drop(storage);

            let status_label = match a.status {
                AmbassadorStatus::Pending => "Pending",
                AmbassadorStatus::Approved => "Approved",
                AmbassadorStatus::Rejected => "Rejected",
            };

            let embed = CreateEmbed::new()
                .title(format!("Ambassador Profile — {}", a.name))
                .description(format!("Discord ID: {}", discord_id))
                .field("Status", status_label, true)
                .field("Points", a.points.to_string(), true)
                .field("School", a.school.as_deref().unwrap_or("None"), true)
                .field("Grade", a.grade.as_deref().unwrap_or("None"), true)
                .field("Created", a.created_at.format("%Y-%m-%d").to_string(), true);

            ctx.send(poise::CreateReply::default().embed(embed)).await?;
        }
        None => {
            ctx.send(
                poise::CreateReply::default()
                    .ephemeral(true)
                    .content("No ambassador profile found for this user."),
            )
            .await?;
        }
    }

    Ok(())
}

#[poise::command(slash_command)]
pub async fn school(
    ctx: Context<'_>,
    #[description = "School name to set"] school: Option<String>,
) -> Result<(), Error> {
    let discord_id = ctx.author().id.get();

    match school {
        Some(name) => {
            let mut storage = ctx.data().storage.lock().await;
            let idx = match storage.find_ambassador_by_discord(discord_id) {
                Some(i) => i,
                None => {
                    drop(storage);
                    ctx.send(
                        poise::CreateReply::default()
                            .ephemeral(true)
                            .content("You are not registered as an ambassador."),
                    )
                    .await?;
                    return Ok(());
                }
            };

            if storage.find_school_by_name(&name).is_none() {
                storage
                    .storage_mut()
                    .schools
                    .push(School::new(name.clone()));
            }

            storage.storage_mut().ambassadors[idx].school = Some(name.clone());
            storage.audit_log(
                discord_id,
                "school".to_string(),
                "ambassador".to_string(),
                format!("School set to {}", name),
            );
            storage.save()?;
            drop(storage);

            ctx.send(
                poise::CreateReply::default()
                    .ephemeral(true)
                    .content(format!("Your school has been set to {}.", name)),
            )
            .await?;
        }
        None => {
            let storage = ctx.data().storage.lock().await;
            let idx = storage.find_ambassador_by_discord(discord_id);
            match idx {
                Some(i) => {
                    let school_name = storage.storage().ambassadors[i]
                        .school
                        .as_deref()
                        .unwrap_or("None")
                        .to_string();
                    drop(storage);
                    ctx.send(
                        poise::CreateReply::default()
                            .ephemeral(true)
                            .content(format!("Your school is {}.", school_name)),
                    )
                    .await?;
                }
                None => {
                    drop(storage);
                    ctx.send(
                        poise::CreateReply::default()
                            .ephemeral(true)
                            .content("You are not registered as an ambassador."),
                    )
                    .await?;
                }
            }
        }
    }

    Ok(())
}

#[poise::command(slash_command)]
pub async fn members(
    ctx: Context<'_>,
    #[description = "Filter by school name"] school: Option<String>,
) -> Result<(), Error> {
    let storage = ctx.data().storage.lock().await;

    let ambassadors: Vec<Ambassador> = match &school {
        Some(name) => storage
            .storage()
            .ambassadors
            .iter()
            .filter(|a| a.school.as_deref() == Some(name.as_str()))
            .cloned()
            .collect(),
        None => storage.storage().ambassadors.clone(),
    };
    drop(storage);

    if ambassadors.is_empty() {
        ctx.send(
            poise::CreateReply::default()
                .ephemeral(true)
                .content("No ambassadors found."),
        )
        .await?;
        return Ok(());
    }

    let mut description = String::new();
    for a in ambassadors.iter() {
        let status_label = match a.status {
            AmbassadorStatus::Pending => "Pending",
            AmbassadorStatus::Approved => "Approved",
            AmbassadorStatus::Rejected => "Rejected",
        };
        description.push_str(&format!(
            "• {} — {} ({} pts)\n",
            a.name, status_label, a.points
        ));
    }

    let title = match &school {
        Some(name) => format!("Ambassadors — {}", name),
        None => "Ambassadors".to_string(),
    };

    let embed = CreateEmbed::new().title(title).description(description);
    ctx.send(poise::CreateReply::default().embed(embed)).await?;
    Ok(())
}

#[poise::command(slash_command)]
pub async fn schools(ctx: Context<'_>) -> Result<(), Error> {
    let storage = ctx.data().storage.lock().await;
    let schools: Vec<School> = storage.storage().schools.clone();
    drop(storage);

    if schools.is_empty() {
        ctx.send(
            poise::CreateReply::default()
                .ephemeral(true)
                .content("No schools found."),
        )
        .await?;
        return Ok(());
    }

    let mut description = String::new();
    for s in schools.iter() {
        description.push_str(&format!(
            "• {} ({} ambassadors)\n",
            s.name, s.ambassador_count
        ));
    }

    let embed = CreateEmbed::new().title("Schools").description(description);
    ctx.send(poise::CreateReply::default().embed(embed)).await?;
    Ok(())
}

#[poise::command(slash_command)]
pub async fn team(ctx: Context<'_>) -> Result<(), Error> {
    let storage = ctx.data().storage.lock().await;
    let ambassadors = storage.storage().ambassadors.clone();
    drop(storage);

    let total = ambassadors.len() as u32;
    let approved = ambassadors
        .iter()
        .filter(|a| matches!(a.status, AmbassadorStatus::Approved))
        .count() as u32;
    let pending = ambassadors
        .iter()
        .filter(|a| matches!(a.status, AmbassadorStatus::Pending))
        .count() as u32;
    let rejected = ambassadors
        .iter()
        .filter(|a| matches!(a.status, AmbassadorStatus::Rejected))
        .count() as u32;
    let total_points: u32 = ambassadors.iter().map(|a| a.points).sum();

    let embed = CreateEmbed::new()
        .title("Ambassador Team")
        .field("Total", total.to_string(), true)
        .field("Approved", approved.to_string(), true)
        .field("Pending", pending.to_string(), true)
        .field("Rejected", rejected.to_string(), true)
        .field("Total Points", total_points.to_string(), true);

    ctx.send(poise::CreateReply::default().embed(embed)).await?;
    Ok(())
}

#[poise::command(slash_command)]
pub async fn activity(
    ctx: Context<'_>,
    #[description = "User to view"] user: Option<serenity::UserId>,
) -> Result<(), Error> {
    let target = user.unwrap_or_else(|| ctx.author().id);
    let discord_id = target.get();

    let storage = ctx.data().storage.lock().await;
    let activities: Vec<ActivityRecord> = storage
        .storage()
        .activities
        .iter()
        .filter(|a| a.discord_id == discord_id)
        .cloned()
        .collect();
    drop(storage);

    if activities.is_empty() {
        ctx.send(
            poise::CreateReply::default()
                .ephemeral(true)
                .content("No activity records found."),
        )
        .await?;
        return Ok(());
    }

    let mut description = String::new();
    for a in activities.iter() {
        description.push_str(&format!(
            "• {} — {} pts ({})\n",
            a.action,
            a.points,
            a.created_at.format("%Y-%m-%d")
        ));
    }

    let embed = CreateEmbed::new()
        .title(format!("Activity — <@{}>", discord_id))
        .description(description);
    ctx.send(poise::CreateReply::default().embed(embed)).await?;
    Ok(())
}

#[poise::command(slash_command)]
pub async fn leaderboard(ctx: Context<'_>) -> Result<(), Error> {
    let storage = ctx.data().storage.lock().await;
    let mut ambassadors: Vec<_> = storage.storage().ambassadors.clone();
    drop(storage);

    ambassadors.sort_by(|a, b| b.points.cmp(&a.points));
    ambassadors.truncate(10);

    if ambassadors.is_empty() {
        ctx.send(
            poise::CreateReply::default()
                .ephemeral(true)
                .content("No ambassadors found."),
        )
        .await?;
        return Ok(());
    }

    let mut description = String::new();
    for (i, a) in ambassadors.iter().enumerate() {
        description.push_str(&format!("{}. {} — {} pts\n", i + 1, a.name, a.points));
    }

    let embed = CreateEmbed::new()
        .title("Ambassador Leaderboard")
        .description(description);
    ctx.send(poise::CreateReply::default().embed(embed)).await?;
    Ok(())
}
