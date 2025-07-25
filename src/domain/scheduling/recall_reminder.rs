// Domain entity for Recall Reminder, modeled after VistA/MUMPS File #409.68
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecallReminder {
    pub patient_id: u32,             // .01 PATIENT (pointer)
    pub recall_date: String,         // .02 RECALL DATE
    pub clinic_id: Option<u32>,      // .03 CLINIC (pointer)
    pub reason: Option<String>,      // .04 REASON
    pub status: Option<String>,      // .05 STATUS
}
