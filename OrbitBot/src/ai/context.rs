use crate::models::{AIConversation, AIMessage};
use crate::storage::Store;
use chrono::Utc;

pub fn get_or_create(storage: &mut Store, discord_id: u64) -> AIConversation {
    if let Some(idx) = storage.find_ai_conversation(discord_id) {
        storage.storage().ai_conversations[idx].clone()
    } else {
        AIConversation {
            discord_id,
            messages: Vec::new(),
            updated_at: Utc::now(),
        }
    }
}

pub fn add_message(
    storage: &mut Store,
    discord_id: u64,
    role: String,
    content: String,
    context_message_limit: usize,
) {
    let conv = match storage.find_ai_conversation(discord_id) {
        Some(idx) => &mut storage.storage_mut().ai_conversations[idx],
        None => {
            storage.storage_mut().ai_conversations.push(AIConversation {
                discord_id,
                messages: Vec::new(),
                updated_at: Utc::now(),
            });
            storage.storage_mut().ai_conversations.last_mut().unwrap()
        }
    };

    conv.messages.push(AIMessage { role, content });
    conv.updated_at = Utc::now();

    while conv.messages.len() > context_message_limit {
        conv.messages.remove(0);
    }
}

pub fn clear(storage: &mut Store, discord_id: u64) -> bool {
    if let Some(idx) = storage.find_ai_conversation(discord_id) {
        storage.storage_mut().ai_conversations.remove(idx);
        true
    } else {
        false
    }
}
