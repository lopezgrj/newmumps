// Domain entity for Patient Discharge Follow-up (custom)
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatientDischargeFollowUp {
    pub id: u32, // .01 FOLLOW-UP ID
    pub patient_id: u32, // .02 PATIENT (pointer)
    pub follow_up_date: String, // .03 FOLLOW-UP DATE
    pub outcome: Option<String>, // .04 OUTCOME
    pub notes: Option<String>, // .05 NOTES
}
