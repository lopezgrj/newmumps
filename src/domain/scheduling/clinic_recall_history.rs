// Domain entity for Clinic Recall History (VistA/MUMPS File #403.56)
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClinicRecallHistory {
    pub id: u32, // .01 HISTORY ID
    pub recall_reminder_id: u32, // .02 RECALL REMINDER (pointer)
    pub action: String, // .03 ACTION
    pub action_date: String, // .04 ACTION DATE
    pub user_id: Option<u32>, // .05 USER (pointer)
}
