use crate::storage::Store;
use std::sync::Arc;
use tokio::sync::Mutex;

pub struct Data {
    pub storage: Arc<Mutex<Store>>,
}

pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Context<'a> = poise::Context<'a, Data, Error>;
