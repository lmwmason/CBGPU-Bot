use crate::storage::Store;
use std::sync::Arc;
use tokio::sync::Mutex;

pub struct Data {
    pub storage: Arc<Mutex<Store>>,
    /// Users with an AI request currently running (prevents cooldown bypass).
    pub ai_in_flight: Arc<std::sync::Mutex<std::collections::HashSet<u64>>>,
}

/// Removes the user from the in-flight set when dropped.
pub struct InFlightGuard {
    set: Arc<std::sync::Mutex<std::collections::HashSet<u64>>>,
    id: u64,
}

impl Data {
    pub fn try_begin_ai(&self, id: u64) -> Option<InFlightGuard> {
        let mut set = self.ai_in_flight.lock().ok()?;
        if set.insert(id) {
            Some(InFlightGuard { set: self.ai_in_flight.clone(), id })
        } else {
            None
        }
    }
}

impl Drop for InFlightGuard {
    fn drop(&mut self) {
        if let Ok(mut set) = self.set.lock() {
            set.remove(&self.id);
        }
    }
}

pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Context<'a> = poise::Context<'a, Data, Error>;
