// Domain entity for Patient ADT Notification Log (custom)
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatientAdtNotificationLog {
    pub id: u32, // .01 LOG ID
    pub patient_id: u32, // .02 PATIENT (pointer)
    pub notification_date: String, // .03 NOTIFICATION DATE
    pub notification_type: String, // .04 NOTIFICATION TYPE
    pub user_id: Option<u32>, // .05 USER (pointer)
    pub notes: Option<String>, // .06 NOTES
}
