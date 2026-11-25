use crate::models::*;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

/// Schools marked "CBGPU 운영 학교" (status APPROVED) on https://cbgpu.com/explore/schools.
/// Checked against https://cbgpu.com/api/public/schools on 2026-11-25.
const CBGPU_OPERATING_SCHOOLS: &[&str] = &["충북과학고등학교"];

const STORAGE_PATH: &str = "data/cbgpu-bot.json";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Storage {
    pub ambassadors: Vec<Ambassador>,
    pub schools: Vec<School>,
    pub missions: Vec<Mission>,
    pub mission_participations: Vec<MissionParticipation>,
    pub submissions: Vec<Submission>,
    pub activities: Vec<ActivityRecord>,
    pub audit_logs: Vec<AuditLog>,
    pub ai_config: Option<AIConfig>,
    pub ai_conversations: Vec<AIConversation>,
    pub ai_usage: Vec<AIUsageRecord>,
    #[serde(default)]
    pub registrations: Vec<RegistrationRecord>,
}

impl Default for Storage {
    fn default() -> Self {
        Self {
            ambassadors: Vec::new(),
            schools: Vec::new(),
            missions: Vec::new(),
            mission_participations: Vec::new(),
            submissions: Vec::new(),
            activities: Vec::new(),
            audit_logs: Vec::new(),
            ai_config: None,
            ai_conversations: Vec::new(),
            ai_usage: Vec::new(),
            registrations: Vec::new(),
        }
    }
}

pub struct Store {
    path: String,
    storage: Storage,
}

impl Store {
    pub fn load() -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        let path = STORAGE_PATH.to_string();
        let storage = if Path::new(&path).exists() {
            let data = fs::read_to_string(&path)?;
            serde_json::from_str(&data)?
        } else {
            Storage::default()
        };
        let mut store = Self { path, storage };
        store.seed_cbgpu_schools();
        Ok(store)
    }

    /// Makes sure the known CBGPU operating schools exist and are marked as participating.
    fn seed_cbgpu_schools(&mut self) {
        for name in CBGPU_OPERATING_SCHOOLS {
            match self.find_school_by_name(name) {
                Some(i) => self.storage.schools[i].cbgu_participating = true,
                None => {
                    let mut school = School::new((*name).to_string());
                    school.cbgu_participating = true;
                    self.storage.schools.push(school);
                }
            }
        }
    }

    /// Creates a default AI config from env vars if none is stored yet.
    /// Needs AI_BASE_URL and AI_MODEL; the key is read from AI_API_KEY at request time.
    pub fn ensure_ai_config_from_env(&mut self) -> bool {
        if self.storage.ai_config.is_some() {
            return false;
        }
        let (Ok(base_url), Ok(model)) = (
            std::env::var("AI_BASE_URL"),
            std::env::var("AI_MODEL"),
        ) else {
            return false;
        };
        let provider = AIProviderConfig {
            name: "default".to_string(),
            base_url,
            model: model.clone(),
            api_key_env: "AI_API_KEY".to_string(),
        };
        self.storage.ai_config = Some(AIConfig {
            providers: vec![provider],
            default_provider: Some("default".to_string()),
            default_model: Some(model),
            enabled: true,
            max_output_tokens: 4096,
            ..AIConfig::default()
        });
        true
    }

    pub fn save(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        if let Some(parent) = Path::new(&self.path).parent() {
            fs::create_dir_all(parent)?;
        }
        let data = serde_json::to_string_pretty(&self.storage)?;
        let temp_path = format!("{}.tmp", self.path);
        fs::write(&temp_path, data)?;
        fs::rename(&temp_path, &self.path)?;
        Ok(())
    }

    pub fn storage(&self) -> &Storage {
        &self.storage
    }

    pub fn storage_mut(&mut self) -> &mut Storage {
        &mut self.storage
    }

    pub fn audit_log(&mut self, actor_id: u64, action: String, target: String, details: String) {
        self.storage
            .audit_logs
            .push(AuditLog::new(actor_id, action, target, details));
    }

    pub fn find_ambassador_by_discord(&self, discord_id: u64) -> Option<usize> {
        self.storage
            .ambassadors
            .iter()
            .position(|a| a.discord_id == discord_id)
    }

    pub fn find_mission(&self, id: &EntityId) -> Option<usize> {
        self.storage.missions.iter().position(|m| &m.id == id)
    }

    pub fn find_mission_participation(
        &self,
        mission_id: &EntityId,
        discord_id: u64,
    ) -> Option<usize> {
        self.storage
            .mission_participations
            .iter()
            .position(|p| p.mission_id == *mission_id && p.discord_id == discord_id)
    }

    pub fn find_submission(&self, id: &EntityId) -> Option<usize> {
        self.storage.submissions.iter().position(|s| &s.id == id)
    }

    pub fn find_submission_by_mission_and_user(
        &self,
        mission_id: &EntityId,
        discord_id: u64,
    ) -> Option<usize> {
        self.storage
            .submissions
            .iter()
            .position(|s| s.mission_id == *mission_id && s.discord_id == discord_id)
    }

    pub fn find_school_by_name(&self, name: &str) -> Option<usize> {
        self.storage
            .schools
            .iter()
            .position(|s| s.name.eq_ignore_ascii_case(name))
    }

    pub fn find_ai_conversation(&self, discord_id: u64) -> Option<usize> {
        self.storage
            .ai_conversations
            .iter()
            .position(|c| c.discord_id == discord_id)
    }

    pub fn find_ai_usage(&self, discord_id: u64) -> Option<usize> {
        self.storage
            .ai_usage
            .iter()
            .position(|u| u.discord_id == discord_id)
    }
}
