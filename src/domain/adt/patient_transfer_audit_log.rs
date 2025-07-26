// Domain entity for Patient Transfer Audit Log (custom)
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatientTransferAuditLog {
    pub id: u32, // .01 LOG ID
    pub patient_id: u32, // .02 PATIENT (pointer)
    pub transfer_id: u32, // .03 TRANSFER (pointer)
    pub action: String, // .04 ACTION
    pub action_date: String, // .05 ACTION DATE
    pub user_id: Option<u32>, // .06 USER (pointer)
    pub notes: Option<String>, // .07 NOTES
}
