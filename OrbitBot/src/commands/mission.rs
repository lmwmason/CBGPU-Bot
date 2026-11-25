use crate::models::*;
use crate::state::{Context, Error};
use chrono::Utc;
use poise::serenity_prelude as serenity;
use serenity::CreateEmbed;
use uuid::Uuid;

fn parse_date(s: &str) -> Result<chrono::DateTime<Utc>, Error> {
    let naive = chrono::NaiveDate::parse_from_str(s, "%Y-%m-%d")?
        .and_hms_opt(0, 0, 0)
        .ok_or("Invalid date")?;
    Ok(naive.and_utc())
}

fn parse_uuid(s: &str) -> Result<EntityId, Error> {
    Ok(Uuid::parse_str(s)?)
}

async fn admin_check(ctx: Context<'_>) -> Result<bool, Error> {
    let cache = ctx.serenity_context().cache.clone();
    let member = ctx.author_member().await;
    Ok(member.map_or(false, |m| {
        m.permissions(cache).map_or(false, |p| p.administrator())
    }))
}

#[poise::command(slash_command)]
pub async fn mission_create(
    ctx: Context<'_>,
    #[description = "Mission title"] title: String,
    #[description = "Mission description"] description: String,
    #[description = "Mission instructions"] instructions: String,
    #[description = "Start date (YYYY-MM-DD)"] start_date: String,
    #[description = "End date (YYYY-MM-DD)"] end_date: String,
    #[description = "Points awarded"] points: u32,
    #[description = "Submission requirements"] submission_requirements: String,
) -> Result<(), Error> {
    let discord_id = ctx.author().id.get();

    if !admin_check(ctx).await? {
        ctx.send(
            poise::CreateReply::default()
                .ephemeral(true)
                .content("Only administrators can create missions."),
        )
        .await?;
        return Ok(());
    }

    let start = parse_date(&start_date)?;
    let end = parse_date(&end_date)?;

    let mut storage = ctx.data().storage.lock().await;

    let mission = Mission::new(
        title,
        description,
        instructions,
        start,
        end,
        points,
        submission_requirements,
        discord_id,
    );
    storage.storage_mut().missions.push(mission);
    storage.audit_log(
        discord_id,
        "mission_create".to_string(),
        "mission".to_string(),
        "Created a new mission".to_string(),
    );
    storage.save()?;
    drop(storage);

    ctx.send(
        poise::CreateReply::default()
            .ephemeral(true)
            .content("Mission created successfully."),
    )
    .await?;
    Ok(())
}

#[poise::command(slash_command)]
pub async fn mission_edit(
    ctx: Context<'_>,
    #[description = "Mission ID"] mission_id: String,
    #[description = "New title"] title: Option<String>,
    #[description = "New description"] description: Option<String>,
    #[description = "New instructions"] instructions: Option<String>,
    #[description = "New start date (YYYY-MM-DD)"] start_date: Option<String>,
    #[description = "New end date (YYYY-MM-DD)"] end_date: Option<String>,
    #[description = "New points"] points: Option<u32>,
    #[description = "New submission requirements"] submission_requirements: Option<String>,
) -> Result<(), Error> {
    let discord_id = ctx.author().id.get();

    if !admin_check(ctx).await? {
        ctx.send(
            poise::CreateReply::default()
                .ephemeral(true)
                .content("Only administrators can edit missions."),
        )
        .await?;
        return Ok(());
    }

    let id = parse_uuid(&mission_id)?;

    let mut storage = ctx.data().storage.lock().await;

    let idx = match storage.find_mission(&id) {
        Some(i) => i,
        None => {
            drop(storage);
            ctx.send(
                poise::CreateReply::default()
                    .ephemeral(true)
                    .content("Mission not found."),
            )
            .await?;
            return Ok(());
        }
    };

    let mission = &mut storage.storage_mut().missions[idx];

    if let Some(t) = title {
        mission.title = t;
    }
    if let Some(d) = description {
        mission.description = d;
    }
    if let Some(i) = instructions {
        mission.instructions = i;
    }
    if let Some(s) = start_date {
        mission.start_date = parse_date(&s)?;
    }
    if let Some(e) = end_date {
        mission.end_date = parse_date(&e)?;
    }
    if let Some(p) = points {
        mission.points = p;
    }
    if let Some(r) = submission_requirements {
        mission.submission_requirements = r;
    }

    storage.audit_log(
        discord_id,
        "mission_edit".to_string(),
        "mission".to_string(),
        format!("Edited mission {}", mission_id),
    );
    storage.save()?;
    drop(storage);

    ctx.send(
        poise::CreateReply::default()
            .ephemeral(true)
            .content("Mission updated successfully."),
    )
    .await?;
    Ok(())
}

#[poise::command(slash_command)]
pub async fn mission_close(
    ctx: Context<'_>,
    #[description = "Mission ID"] mission_id: String,
) -> Result<(), Error> {
    let discord_id = ctx.author().id.get();

    if !admin_check(ctx).await? {
        ctx.send(
            poise::CreateReply::default()
                .ephemeral(true)
                .content("Only administrators can close missions."),
        )
        .await?;
        return Ok(());
    }

    let id = parse_uuid(&mission_id)?;

    let mut storage = ctx.data().storage.lock().await;

    let idx = match storage.find_mission(&id) {
        Some(i) => i,
        None => {
            drop(storage);
            ctx.send(
                poise::CreateReply::default()
                    .ephemeral(true)
                    .content("Mission not found."),
            )
            .await?;
            return Ok(());
        }
    };

    storage.storage_mut().missions[idx].status = MissionStatus::Cancelled;
    storage.audit_log(
        discord_id,
        "mission_close".to_string(),
        "mission".to_string(),
        format!("Closed mission {}", mission_id),
    );
    storage.save()?;
    drop(storage);

    ctx.send(
        poise::CreateReply::default()
            .ephemeral(true)
            .content("Mission closed successfully."),
    )
    .await?;
    Ok(())
}

#[poise::command(slash_command)]
pub async fn mission_delete(
    ctx: Context<'_>,
    #[description = "Mission ID"] mission_id: String,
) -> Result<(), Error> {
    let discord_id = ctx.author().id.get();

    if !admin_check(ctx).await? {
        ctx.send(
            poise::CreateReply::default()
                .ephemeral(true)
                .content("Only administrators can delete missions."),
        )
        .await?;
        return Ok(());
    }

    let id = parse_uuid(&mission_id)?;

    let mut storage = ctx.data().storage.lock().await;

    let idx = match storage.find_mission(&id) {
        Some(i) => i,
        None => {
            drop(storage);
            ctx.send(
                poise::CreateReply::default()
                    .ephemeral(true)
                    .content("Mission not found."),
            )
            .await?;
            return Ok(());
        }
    };

    storage.storage_mut().missions.remove(idx);
    storage.audit_log(
        discord_id,
        "mission_delete".to_string(),
        "mission".to_string(),
        format!("Deleted mission {}", mission_id),
    );
    storage.save()?;
    drop(storage);

    ctx.send(
        poise::CreateReply::default()
            .ephemeral(true)
            .content("Mission deleted successfully."),
    )
    .await?;
    Ok(())
}

#[poise::command(slash_command)]
pub async fn missions(ctx: Context<'_>) -> Result<(), Error> {
    let storage = ctx.data().storage.lock().await;
    let mission_list: Vec<Mission> = storage.storage().missions.clone();
    drop(storage);

    if mission_list.is_empty() {
        ctx.send(
            poise::CreateReply::default()
                .ephemeral(true)
                .content("No missions found."),
        )
        .await?;
        return Ok(());
    }

    let mut description = String::new();
    for m in mission_list.iter() {
        let status_label = match m.status {
            MissionStatus::Active => "Active",
            MissionStatus::Completed => "Completed",
            MissionStatus::Cancelled => "Cancelled",
        };
        description.push_str(&format!(
            "• {} — {} pts [{}]\n  {}\n",
            m.title, m.points, status_label, m.id
        ));
    }

    let embed = CreateEmbed::new()
        .title("Missions")
        .description(description);
    ctx.send(poise::CreateReply::default().embed(embed)).await?;
    Ok(())
}

#[poise::command(slash_command)]
pub async fn mission(
    ctx: Context<'_>,
    #[description = "Mission ID"] mission_id: String,
) -> Result<(), Error> {
    let id = parse_uuid(&mission_id)?;

    let storage = ctx.data().storage.lock().await;
    let idx = storage.find_mission(&id);
    drop(storage);

    match idx {
        Some(i) => {
            let storage = ctx.data().storage.lock().await;
            let m = storage.storage().missions[i].clone();
            drop(storage);

            let status_label = match m.status {
                MissionStatus::Active => "Active",
                MissionStatus::Completed => "Completed",
                MissionStatus::Cancelled => "Cancelled",
            };

            let embed = CreateEmbed::new()
                .title(format!("Mission — {}", m.title))
                .field("Description", m.description, false)
                .field("Instructions", m.instructions, false)
                .field("Points", m.points.to_string(), true)
                .field("Status", status_label, true)
                .field("Start", m.start_date.format("%Y-%m-%d").to_string(), true)
                .field("End", m.end_date.format("%Y-%m-%d").to_string(), true)
                .field("Requirements", m.submission_requirements, false)
                .field("ID", m.id.to_string(), false);

            ctx.send(poise::CreateReply::default().embed(embed)).await?;
        }
        None => {
            ctx.send(
                poise::CreateReply::default()
                    .ephemeral(true)
                    .content("Mission not found."),
            )
            .await?;
        }
    }

    Ok(())
}

#[poise::command(slash_command)]
pub async fn mission_participate(
    ctx: Context<'_>,
    #[description = "Mission ID"] mission_id: String,
) -> Result<(), Error> {
    let discord_id = ctx.author().id.get();
    let id = parse_uuid(&mission_id)?;

    let mut storage = ctx.data().storage.lock().await;

    let mission_idx = match storage.find_mission(&id) {
        Some(i) => i,
        None => {
            drop(storage);
            ctx.send(
                poise::CreateReply::default()
                    .ephemeral(true)
                    .content("Mission not found."),
            )
            .await?;
            return Ok(());
        }
    };

    let mission = storage.storage().missions[mission_idx].clone();
    if mission.status != MissionStatus::Active {
        drop(storage);
        ctx.send(
            poise::CreateReply::default()
                .ephemeral(true)
                .content("This mission is not active."),
        )
        .await?;
        return Ok(());
    }

    if storage.find_ambassador_by_discord(discord_id).is_none() {
        drop(storage);
        ctx.send(
            poise::CreateReply::default()
                .ephemeral(true)
                .content("You are not registered as an ambassador."),
        )
        .await?;
        return Ok(());
    }

    if storage
        .find_mission_participation(&id, discord_id)
        .is_some()
    {
        drop(storage);
        ctx.send(
            poise::CreateReply::default()
                .ephemeral(true)
                .content("You have already joined this mission."),
        )
        .await?;
        return Ok(());
    }

    let participation = MissionParticipation::new(id, discord_id);
    storage
        .storage_mut()
        .mission_participations
        .push(participation);
    storage.audit_log(
        discord_id,
        "mission_participate".to_string(),
        "mission".to_string(),
        format!("Joined mission {}", mission_id),
    );
    storage.save()?;
    drop(storage);

    ctx.send(
        poise::CreateReply::default()
            .ephemeral(true)
            .content(format!(
                "You have joined the mission \"{}\".",
                mission.title
            )),
    )
    .await?;
    Ok(())
}

#[poise::command(slash_command)]
pub async fn mission_submit(
    ctx: Context<'_>,
    #[description = "Mission ID"] mission_id: String,
    #[description = "Submission content"] content: String,
) -> Result<(), Error> {
    let discord_id = ctx.author().id.get();
    let id = parse_uuid(&mission_id)?;

    let mut storage = ctx.data().storage.lock().await;

    let mission_idx = match storage.find_mission(&id) {
        Some(i) => i,
        None => {
            drop(storage);
            ctx.send(
                poise::CreateReply::default()
                    .ephemeral(true)
                    .content("Mission not found."),
            )
            .await?;
            return Ok(());
        }
    };

    let mission = storage.storage().missions[mission_idx].clone();
    if mission.status != MissionStatus::Active {
        drop(storage);
        ctx.send(
            poise::CreateReply::default()
                .ephemeral(true)
                .content("This mission is not active."),
        )
        .await?;
        return Ok(());
    }

    if storage
        .find_mission_participation(&id, discord_id)
        .is_none()
    {
        drop(storage);
        ctx.send(
            poise::CreateReply::default()
                .ephemeral(true)
                .content("You must join this mission before submitting."),
        )
        .await?;
        return Ok(());
    }

    if storage
        .find_submission_by_mission_and_user(&id, discord_id)
        .is_some()
    {
        drop(storage);
        ctx.send(
            poise::CreateReply::default()
                .ephemeral(true)
                .content("You have already submitted for this mission."),
        )
        .await?;
        return Ok(());
    }

    let submission = Submission::new(id, discord_id, content);
    storage.storage_mut().submissions.push(submission);
    storage.audit_log(
        discord_id,
        "mission_submit".to_string(),
        "mission".to_string(),
        format!("Submitted for mission {}", mission_id),
    );
    storage.save()?;
    drop(storage);

    ctx.send(
        poise::CreateReply::default()
            .ephemeral(true)
            .content(format!(
                "Your submission for \"{}\" has been recorded and is pending review.",
                mission.title
            )),
    )
    .await?;
    Ok(())
}

#[poise::command(slash_command)]
pub async fn submission_approve(
    ctx: Context<'_>,
    #[description = "Submission ID"] submission_id: String,
    #[description = "Points awarded"] points_awarded: u32,
) -> Result<(), Error> {
    let discord_id = ctx.author().id.get();
    let id = parse_uuid(&submission_id)?;

    if !admin_check(ctx).await? {
        ctx.send(
            poise::CreateReply::default()
                .ephemeral(true)
                .content("Only administrators can approve submissions."),
        )
        .await?;
        return Ok(());
    }

    let mut storage = ctx.data().storage.lock().await;

    let idx = match storage.find_submission(&id) {
        Some(i) => i,
        None => {
            drop(storage);
            ctx.send(
                poise::CreateReply::default()
                    .ephemeral(true)
                    .content("Submission not found."),
            )
            .await?;
            return Ok(());
        }
    };

    let submission = storage.storage().submissions[idx].clone();
    if submission.status != SubmissionStatus::Pending {
        drop(storage);
        ctx.send(
            poise::CreateReply::default()
                .ephemeral(true)
                .content("This submission has already been reviewed."),
        )
        .await?;
        return Ok(());
    }

    storage.storage_mut().submissions[idx].status = SubmissionStatus::Approved;
    storage.storage_mut().submissions[idx].reviewed_at = Some(Utc::now());
    storage.storage_mut().submissions[idx].reviewed_by = Some(discord_id);
    storage.storage_mut().submissions[idx].review_reason = None;
    storage.storage_mut().submissions[idx].points_awarded = Some(points_awarded);

    let activity = ActivityRecord::new(
        submission.discord_id,
        format!("Approved submission for mission {}", submission.mission_id),
        points_awarded,
    );
    storage.storage_mut().activities.push(activity);

    storage.audit_log(
        discord_id,
        "submission_approve".to_string(),
        "submission".to_string(),
        format!(
            "Approved submission {} with {} points",
            submission_id, points_awarded
        ),
    );
    storage.save()?;
    drop(storage);

    ctx.send(
        poise::CreateReply::default()
            .ephemeral(true)
            .content(format!(
                "Submission approved. {} points awarded to user {}.",
                points_awarded, submission.discord_id
            )),
    )
    .await?;
    Ok(())
}

#[poise::command(slash_command)]
pub async fn submission_reject(
    ctx: Context<'_>,
    #[description = "Submission ID"] submission_id: String,
    #[description = "Rejection reason"] review_reason: String,
) -> Result<(), Error> {
    let discord_id = ctx.author().id.get();
    let id = parse_uuid(&submission_id)?;

    if !admin_check(ctx).await? {
        ctx.send(
            poise::CreateReply::default()
                .ephemeral(true)
                .content("Only administrators can reject submissions."),
        )
        .await?;
        return Ok(());
    }

    let mut storage = ctx.data().storage.lock().await;

    let idx = match storage.find_submission(&id) {
        Some(i) => i,
        None => {
            drop(storage);
            ctx.send(
                poise::CreateReply::default()
                    .ephemeral(true)
                    .content("Submission not found."),
            )
            .await?;
            return Ok(());
        }
    };

    let submission = storage.storage().submissions[idx].clone();
    if submission.status != SubmissionStatus::Pending {
        drop(storage);
        ctx.send(
            poise::CreateReply::default()
                .ephemeral(true)
                .content("This submission has already been reviewed."),
        )
        .await?;
        return Ok(());
    }

    storage.storage_mut().submissions[idx].status = SubmissionStatus::Rejected;
    storage.storage_mut().submissions[idx].reviewed_at = Some(Utc::now());
    storage.storage_mut().submissions[idx].reviewed_by = Some(discord_id);
    storage.storage_mut().submissions[idx].review_reason = Some(review_reason.clone());
    storage.storage_mut().submissions[idx].points_awarded = None;

    storage.audit_log(
        discord_id,
        "submission_reject".to_string(),
        "submission".to_string(),
        format!("Rejected submission {}: {}", submission_id, review_reason),
    );
    storage.save()?;
    drop(storage);

    ctx.send(
        poise::CreateReply::default()
            .ephemeral(true)
            .content("Submission rejected."),
    )
    .await?;
    Ok(())
}

#[poise::command(slash_command)]
pub async fn school_leaderboard(ctx: Context<'_>) -> Result<(), Error> {
    let storage = ctx.data().storage.lock().await;

    let mut school_points: Vec<(String, u32)> = storage
        .storage()
        .schools
        .iter()
        .map(|s| (s.name.clone(), 0u32))
        .collect();

    for a in storage.storage().ambassadors.iter() {
        if let Some(ref school) = a.school {
            if let Some((_, pts)) = school_points.iter_mut().find(|(name, _)| name == school) {
                *pts += a.points;
            }
        }
    }

    school_points.sort_by(|a, b| b.1.cmp(&a.1));
    school_points.truncate(10);
    drop(storage);

    if school_points.is_empty() {
        ctx.send(
            poise::CreateReply::default()
                .ephemeral(true)
                .content("No schools found."),
        )
        .await?;
        return Ok(());
    }

    let mut description = String::new();
    for (i, (name, pts)) in school_points.iter().enumerate() {
        description.push_str(&format!("{}. {} — {} pts\n", i + 1, name, pts));
    }

    let embed = CreateEmbed::new()
        .title("School Leaderboard")
        .description(description);
    ctx.send(poise::CreateReply::default().embed(embed)).await?;
    Ok(())
}
