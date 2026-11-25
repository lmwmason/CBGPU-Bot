use crate::storage::Store;
use chrono::{DateTime, FixedOffset, TimeZone, Utc};
use poise::serenity_prelude as serenity;
use std::collections::HashSet;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Mutex;

const REMINDER_HOUR_KST: u32 = 23;

pub const REMINDER_TEXT: &str = "안녕하세요! 혹시 CBGPU 학교 등록이랑 아직 안 하신 것 같아서 연락드려요!\n\n\
활동 시작하려면 학교 등록이랑 회원가입이 필요해서, 시간 되실 때 가능한 빨리 진행해 주시면 감사하겠습니다!\n\n\
혹시 등록하는 방법이 어렵거나 잘 모르겠어서 못 하고 계신 거라면 편하게 말씀해 주세요! 제가 같이 하나씩 진행하면서 도와드릴게요!!\n\n\
완료하시면 알려주세요! 감사합니다 😊";

#[derive(Debug, Default)]
pub struct ReminderReport {
    pub sent: usize,
    pub failed: Vec<String>,
    pub registered: usize,
}

fn kst() -> FixedOffset {
    FixedOffset::east_opt(9 * 3600).expect("valid offset")
}

/// Time until the next 23:00 Korea time (KST has no DST).
pub fn until_next_reminder(now: DateTime<Utc>) -> Duration {
    let now_kst = now.with_timezone(&kst());
    let today = now_kst
        .date_naive()
        .and_hms_opt(REMINDER_HOUR_KST, 0, 0)
        .expect("valid time");
    let mut next = kst().from_local_datetime(&today).single().expect("fixed offset");
    if next <= now_kst {
        next += chrono::Duration::days(1);
    }
    (next - now_kst)
        .to_std()
        .unwrap_or(Duration::from_secs(60))
}

/// All non-bot members of the guild.
pub async fn fetch_members(
    http: &serenity::Http,
    guild_id: serenity::GuildId,
) -> Result<Vec<serenity::Member>, serenity::Error> {
    let mut all = Vec::new();
    let mut after: Option<serenity::UserId> = None;
    loop {
        let batch = guild_id.members(http, Some(1000), after).await?;
        let n = batch.len();
        after = batch.last().map(|m| m.user.id);
        all.extend(batch.into_iter().filter(|m| !m.user.bot));
        if n < 1000 {
            break;
        }
    }
    Ok(all)
}

/// DMs every member who has not been marked as registered.
pub async fn send_reminders(
    http: &serenity::Http,
    guild_id: serenity::GuildId,
    storage: &Arc<Mutex<Store>>,
) -> Result<ReminderReport, serenity::Error> {
    let members = fetch_members(http, guild_id).await?;
    let registered: HashSet<u64> = {
        let s = storage.lock().await;
        s.storage().registrations.iter().map(|r| r.discord_id).collect()
    };

    let mut report = ReminderReport::default();
    for m in members {
        if registered.contains(&m.user.id.get()) {
            report.registered += 1;
            continue;
        }
        let msg = serenity::CreateMessage::new().content(REMINDER_TEXT);
        match m.user.direct_message(http, msg).await {
            Ok(_) => report.sent += 1,
            Err(_) => report.failed.push(m.display_name().to_string()),
        }
        // Stay well below Discord's rate limits.
        tokio::time::sleep(Duration::from_millis(1200)).await;
    }
    Ok(report)
}

/// Runs forever: every day at 23:00 KST, DM all unregistered members.
pub async fn reminder_loop(
    http: Arc<serenity::Http>,
    guild_id: serenity::GuildId,
    storage: Arc<Mutex<Store>>,
) {
    loop {
        tokio::time::sleep(until_next_reminder(Utc::now())).await;
        match send_reminders(&http, guild_id, &storage).await {
            Ok(r) => println!(
                "Reminders: {} sent, {} failed (DMs closed), {} already registered.",
                r.sent,
                r.failed.len(),
                r.registered
            ),
            Err(e) => eprintln!("Reminder run failed: {e}"),
        }
        // Avoid a double run if the clock lands on 23:00:00 again.
        tokio::time::sleep(Duration::from_secs(5)).await;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn utc(y: i32, mo: u32, d: u32, h: u32, mi: u32) -> DateTime<Utc> {
        Utc.with_ymd_and_hms(y, mo, d, h, mi, 0).unwrap()
    }

    #[test]
    fn next_reminder_same_day() {
        // 2026-11-25 13:00 UTC = 22:00 KST -> 1 hour to 23:00 KST
        assert_eq!(until_next_reminder(utc(2026, 11, 25, 13, 0)), Duration::from_secs(3600));
    }

    #[test]
    fn next_reminder_rolls_to_next_day() {
        // 2026-11-25 14:00 UTC = 23:00 KST exactly -> 24h
        assert_eq!(until_next_reminder(utc(2026, 11, 25, 14, 0)), Duration::from_secs(24 * 3600));
        // 2026-11-25 15:00 UTC = 00:00 KST -> 23h
        assert_eq!(until_next_reminder(utc(2026, 11, 25, 15, 0)), Duration::from_secs(23 * 3600));
    }
}
