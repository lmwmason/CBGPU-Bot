use crate::models::AIUsageRecord;
use crate::storage::Store;
use chrono::Utc;

pub fn record_usage(
    storage: &mut Store,
    discord_id: u64,
    provider: String,
    model: String,
    input_tokens: u32,
    output_tokens: u32,
) {
    let now = Utc::now();
    match storage.find_ai_usage(discord_id) {
        Some(idx) => {
            let record = &mut storage.storage_mut().ai_usage[idx];
            record.request_count += 1;
            record.input_tokens += input_tokens;
            record.output_tokens += output_tokens;
            record.last_request = Some(now);
        }
        None => {
            storage.storage_mut().ai_usage.push(AIUsageRecord {
                discord_id,
                provider,
                model,
                request_count: 1,
                input_tokens,
                output_tokens,
                last_request: Some(now),
            });
        }
    }
}

pub fn check_cooldown(storage: &Store, discord_id: u64, cooldown_secs: u64) -> Option<u64> {
    let record = storage.find_ai_usage(discord_id)?;
    let last = storage.storage().ai_usage[record].last_request?;
    let elapsed = Utc::now().signed_duration_since(last).num_seconds() as u64;
    if elapsed < cooldown_secs {
        Some(cooldown_secs - elapsed)
    } else {
        None
    }
}
