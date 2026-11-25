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
    pub fn new(discord_id: u64, name: String, school: String, grade: Option<String>, introduction: String, reason: String) -> Self {
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
pub enum ApplicationStatus {
    Pending,
    Approved,
    Rejected,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Application {
    pub id: EntityId,
    pub discord_id: u64,
    pub name: String,
    pub school: String,
    pub grade: Option<String>,
    pub introduction: String,
    pub reason: String,
    pub status: ApplicationStatus,
    pub created_at: chrono::DateTime<Utc>,
    pub reviewed_at: Option<chrono::DateTime<Utc>>,
    pub reviewed_by: Option<u64>,
    pub review_reason: Option<String>,
}

impl Application {
    pub fn new(discord_id: u64, name: String, school: String, grade: Option<String>, introduction: String, reason: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            discord_id,
            name,
            school,
            grade,
            introduction,
            reason,
            status: ApplicationStatus::Pending,
            created_at: Utc::now(),
            reviewed_at: None,
            reviewed_by: None,
            review_reason: None,
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

impl Mission {
    pub fn new(title: String, description: String, instructions: String, start_date: chrono::DateTime<Utc>, end_date: chrono::DateTime<Utc>, points: u32, submission_requirements: String, created_by: u64) -> Self {
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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum CollabStatus {
    Open,
    Closed,
    Completed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Collaboration {
    pub id: EntityId,
    pub title: String,
    pub description: String,
    pub required_skills: String,
    pub looking_for: String,
    pub participating_schools: Vec<String>,
    pub contact: String,
    pub status: CollabStatus,
    pub created_by: u64,
    pub created_at: chrono::DateTime<Utc>,
    pub members: Vec<u64>,
}

impl Collaboration {
    pub fn new(title: String, description: String, required_skills: String, looking_for: String, participating_schools: Vec<String>, contact: String, created_by: u64) -> Self {
        Self {
            id: Uuid::new_v4(),
            title,
            description,
            required_skills,
            looking_for,
            participating_schools,
            contact,
            status: CollabStatus::Open,
            created_by,
            created_at: Utc::now(),
            members: vec![created_by],
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum CollabRequestStatus {
    Pending,
    Accepted,
    Rejected,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CollabRequest {
    pub id: EntityId,
    pub collab_id: EntityId,
    pub discord_id: u64,
    pub status: CollabRequestStatus,
    pub created_at: chrono::DateTime<Utc>,
    pub responded_at: Option<chrono::DateTime<Utc>>,
    pub responded_by: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ConnectionStatus {
    Pending,
    Accepted,
    Rejected,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Connection {
    pub id: EntityId,
    pub requester_id: u64,
    pub target_id: u64,
    pub status: ConnectionStatus,
    pub created_at: chrono::DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum EventStatus {
    Upcoming,
    Completed,
    Cancelled,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Event {
    pub id: EntityId,
    pub title: String,
    pub description: String,
    pub event_date: chrono::DateTime<Utc>,
    pub location: String,
    pub status: EventStatus,
    pub created_by: u64,
    pub created_at: chrono::DateTime<Utc>,
    pub rsvps: Vec<u64>,
}

impl Event {
    pub fn new(title: String, description: String, event_date: chrono::DateTime<Utc>, location: String, created_by: u64) -> Self {
        Self {
            id: Uuid::new_v4(),
            title,
            description,
            event_date,
            location,
            status: EventStatus::Upcoming,
            created_by,
            created_at: Utc::now(),
            rsvps: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum TicketCategory {
    SchoolOnboarding,
    PromotionalMaterials,
    CbgpuTechnicalIssues,
    AmbassadorProgramQuestions,
    EventSupport,
    Other,
}

impl TicketCategory {
    pub fn label(&self) -> &'static str {
        match self {
            TicketCategory::SchoolOnboarding => "School onboarding",
            TicketCategory::PromotionalMaterials => "Promotional materials",
            TicketCategory::CbgpuTechnicalIssues => "CBGPU technical issues",
            TicketCategory::AmbassadorProgramQuestions => "Ambassador program questions",
            TicketCategory::EventSupport => "Event support",
            TicketCategory::Other => "Other",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum TicketStatus {
    Open,
    InProgress,
    Closed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ticket {
    pub id: EntityId,
    pub discord_id: u64,
    pub category: TicketCategory,
    pub subject: String,
    pub description: String,
    pub status: TicketStatus,
    pub created_at: chrono::DateTime<Utc>,
    pub closed_at: Option<chrono::DateTime<Utc>>,
    pub closed_by: Option<u64>,
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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum AnnouncementStatus {
    Active,
    Archived,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Announcement {
    pub id: EntityId,
    pub title: String,
    pub content: String,
    pub status: AnnouncementStatus,
    pub created_by: u64,
    pub created_at: chrono::DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum PollStatus {
    Active,
    Closed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Poll {
    pub id: EntityId,
    pub question: String,
    pub options: Vec<String>,
    pub votes: Vec<Vec<u64>>,
    pub status: PollStatus,
    pub created_by: u64,
    pub created_at: chrono::DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum RewardType {
    Role,
    Badge,
    Swag,
    Recognition,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Reward {
    pub id: EntityId,
    pub title: String,
    pub description: String,
    pub points_cost: u32,
    pub reward_type: RewardType,
    pub active: bool,
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
