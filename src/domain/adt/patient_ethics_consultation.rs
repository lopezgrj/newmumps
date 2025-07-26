// Domain entity for Patient Ethics Consultation (custom)
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatientEthicsConsultation {
    pub id: u32, // .01 CONSULTATION ID
    pub patient_id: u32, // .02 PATIENT (pointer)
    pub consultation_date: String, // .03 CONSULTATION DATE
    pub issue: String, // .04 ISSUE
    pub outcome: Option<String>, // .05 OUTCOME
}
