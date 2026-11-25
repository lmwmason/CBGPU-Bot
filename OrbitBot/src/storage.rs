use crate::models::*;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

const STORAGE_PATH: &str = "data/orbitbot.json";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Storage {
    pub ambassadors: Vec<Ambassador>,
    pub schools: Vec<School>,
    pub applications: Vec<Application>,
    pub missions: Vec<Mission>,
    pub mission_participations: Vec<MissionParticipation>,
    pub submissions: Vec<Submission>,
    pub activities: Vec<ActivityRecord>,
    pub collaborations: Vec<Collaboration>,
    pub collab_requests: Vec<CollabRequest>,
    pub connections: Vec<Connection>,
    pub events: Vec<Event>,
    pub tickets: Vec<Ticket>,
    pub audit_logs: Vec<AuditLog>,
    pub announcements: Vec<Announcement>,
    pub polls: Vec<Poll>,
    pub rewards: Vec<Reward>,
}

impl Default for Storage {
    fn default() -> Self {
        Self {
            ambassadors: Vec::new(),
            schools: Vec::new(),
            applications: Vec::new(),
            missions: Vec::new(),
            mission_participations: Vec::new(),
            submissions: Vec::new(),
            activities: Vec::new(),
            collaborations: Vec::new(),
            collab_requests: Vec::new(),
            connections: Vec::new(),
            events: Vec::new(),
            tickets: Vec::new(),
            audit_logs: Vec::new(),
            announcements: Vec::new(),
            polls: Vec::new(),
            rewards: Vec::new(),
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
        Ok(Self { path, storage })
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

    pub fn find_application_by_discord(&self, discord_id: u64) -> Option<usize> {
        self.storage
            .applications
            .iter()
            .position(|a| a.discord_id == discord_id)
    }

    pub fn find_mission(&self, id: &EntityId) -> Option<usize> {
        self.storage.missions.iter().position(|m| &m.id == id)
    }

    pub fn find_mission_participation(&self, mission_id: &EntityId, discord_id: u64) -> Option<usize> {
        self.storage
            .mission_participations
            .iter()
            .position(|p| p.mission_id == *mission_id && p.discord_id == discord_id)
    }

    pub fn find_mission_participations_by_mission(&self, mission_id: &EntityId) -> Vec<usize> {
        self.storage
            .mission_participations
            .iter()
            .enumerate()
            .filter(|(_, p)| p.mission_id == *mission_id)
            .map(|(i, _)| i)
            .collect()
    }

    pub fn find_mission_participations_by_user(&self, discord_id: u64) -> Vec<usize> {
        self.storage
            .mission_participations
            .iter()
            .enumerate()
            .filter(|(_, p)| p.discord_id == discord_id)
            .map(|(i, _)| i)
            .collect()
    }

    pub fn find_submission(&self, id: &EntityId) -> Option<usize> {
        self.storage
            .submissions
            .iter()
            .position(|s| &s.id == id)
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

    pub fn find_collab(&self, id: &EntityId) -> Option<usize> {
        self.storage
            .collaborations
            .iter()
            .position(|c| &c.id == id)
    }

    pub fn find_collab_request(&self, id: &EntityId) -> Option<usize> {
        self.storage
            .collab_requests
            .iter()
            .position(|c| &c.id == id)
    }

    pub fn find_event(&self, id: &EntityId) -> Option<usize> {
        self.storage.events.iter().position(|e| &e.id == id)
    }

    pub fn find_ticket(&self, id: &EntityId) -> Option<usize> {
        self.storage.tickets.iter().position(|t| &t.id == id)
    }

    pub fn find_school_by_name(&self, name: &str) -> Option<usize> {
        self.storage
            .schools
            .iter()
            .position(|s| s.name.eq_ignore_ascii_case(name))
    }

    pub fn ambassador_points(&self, discord_id: u64) -> u32 {
        self.storage
            .ambassadors
            .iter()
            .find(|a| a.discord_id == discord_id)
            .map(|a| a.points)
            .unwrap_or(0)
    }
}
