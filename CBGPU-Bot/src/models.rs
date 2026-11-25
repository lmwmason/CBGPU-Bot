use chrono::Utc;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub type EntityId = Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum AmbassadorStatus {
    Pending,
    Approved,
    Rejected,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ambassador {
    pub id: EntityId,
    pub discord_id: u64,
    pub name: String,
    pub school: Option<String>,
    pub grade: Option<String>,
    pub introduction: Option<String>,
    pub reason: Option<String>,
    pub status: AmbassadorStatus,
    pub points: u32,
    pub created_at: chrono::DateTime<Utc>,
    pub approved_at: Option<chrono::DateTime<Utc>>,
    pub approved_by: Option<u64>,
}

impl Ambassador {
    pub fn new(
        discord_id: u64,
        name: String,
        school: String,
        grade: Option<String>,
        introduction: String,
        reason: String,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            discord_id,
            name,
            school: Some(school),
            grade,
            introduction: Some(introduction),
            reason: Some(reason),
            status: AmbassadorStatus::Pending,
            points: 0,
            created_at: Utc::now(),
            approved_at: None,
            approved_by: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum MissionStatus {
    Active,
    Completed,
    Cancelled,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Mission {
    pub id: EntityId,
    pub title: String,
    pub description: String,
    pub instructions: String,
    pub start_date: chrono::DateTime<Utc>,
    pub end_date: chrono::DateTime<Utc>,
    pub points: u32,
    pub submission_requirements: String,
    pub status: MissionStatus,
    pub created_by: u64,
    pub created_at: chrono::DateTime<Utc>,
}

#[allow(clippy::too_many_arguments)]
impl Mission {
    pub fn new(
        title: String,
        description: String,
        instructions: String,
        start_date: chrono::DateTime<Utc>,
        end_date: chrono::DateTime<Utc>,
        points: u32,
        submission_requirements: String,
        created_by: u64,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            title,
            description,
            instructions,
            start_date,
            end_date,
            points,
            submission_requirements,
            status: MissionStatus::Active,
            created_by,
            created_at: Utc::now(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum SubmissionStatus {
    Pending,
    Approved,
    Rejected,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MissionParticipation {
    pub id: EntityId,
    pub mission_id: EntityId,
    pub discord_id: u64,
    pub joined_at: chrono::DateTime<Utc>,
}

impl MissionParticipation {
    pub fn new(mission_id: EntityId, discord_id: u64) -> Self {
        Self {
            id: Uuid::new_v4(),
            mission_id,
            discord_id,
            joined_at: Utc::now(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Submission {
    pub id: EntityId,
    pub mission_id: EntityId,
    pub discord_id: u64,
    pub content: String,
    pub status: SubmissionStatus,
    pub submitted_at: chrono::DateTime<Utc>,
    pub reviewed_at: Option<chrono::DateTime<Utc>>,
    pub reviewed_by: Option<u64>,
    pub review_reason: Option<String>,
    pub points_awarded: Option<u32>,
}

impl Submission {
    pub fn new(mission_id: EntityId, discord_id: u64, content: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            mission_id,
            discord_id,
            content,
            status: SubmissionStatus::Pending,
            submitted_at: Utc::now(),
            reviewed_at: None,
            reviewed_by: None,
            review_reason: None,
            points_awarded: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActivityRecord {
    pub id: EntityId,
    pub discord_id: u64,
    pub action: String,
    pub points: u32,
    pub created_at: chrono::DateTime<Utc>,
}

impl ActivityRecord {
    pub fn new(discord_id: u64, action: String, points: u32) -> Self {
        Self {
            id: Uuid::new_v4(),
            discord_id,
            action,
            points,
            created_at: Utc::now(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditLog {
    pub id: EntityId,
    pub actor_id: u64,
    pub action: String,
    pub target: String,
    pub details: String,
    pub created_at: chrono::DateTime<Utc>,
}

impl AuditLog {
    pub fn new(actor_id: u64, action: String, target: String, details: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            actor_id,
            action,
            target,
            details,
            created_at: Utc::now(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct School {
    pub id: EntityId,
    pub name: String,
    pub ambassador_count: u32,
    #[serde(default)]
    pub cbgu_participating: bool,
    pub created_at: chrono::DateTime<Utc>,
}

impl School {
    pub fn new(name: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            ambassador_count: 0,
            cbgu_participating: false,
            created_at: Utc::now(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIProviderConfig {
    pub name: String,
    pub base_url: String,
    pub model: String,
    pub api_key_env: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIConfig {
    pub providers: Vec<AIProviderConfig>,
    pub default_provider: Option<String>,
    pub default_model: Option<String>,
    pub enabled: bool,
    pub context_message_limit: usize,
    pub max_input_tokens: usize,
    pub max_output_tokens: usize,
    pub request_timeout_secs: u64,
    pub cooldown_secs: u64,
    pub max_concurrent_requests: usize,
}

impl Default for AIConfig {
    fn default() -> Self {
        Self {
            providers: Vec::new(),
            default_provider: None,
            default_model: None,
            enabled: false,
            context_message_limit: 20,
            max_input_tokens: 4000,
            max_output_tokens: 2000,
            request_timeout_secs: 30,
            cooldown_secs: 5,
            max_concurrent_requests: 5,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIMessage {
    pub role: String,
    pub content: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIConversation {
    pub discord_id: u64,
    pub messages: Vec<AIMessage>,
    pub updated_at: chrono::DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIUsageRecord {
    pub discord_id: u64,
    pub provider: String,
    pub model: String,
    pub request_count: u32,
    pub input_tokens: u32,
    pub output_tokens: u32,
    pub last_request: Option<chrono::DateTime<Utc>>,
}

/// A member an admin has confirmed as having registered their school on CBGPU.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegistrationRecord {
    pub discord_id: u64,
    pub school: Option<String>,
    pub marked_by: u64,
    pub marked_at: chrono::DateTime<Utc>,
}
