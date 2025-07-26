// Domain entity for Patient Social Work Referral (custom)
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatientSocialWorkReferral {
    pub id: u32, // .01 REFERRAL ID
    pub patient_id: u32, // .02 PATIENT (pointer)
    pub referral_date: String, // .03 REFERRAL DATE
    pub reason: String, // .04 REASON
    pub outcome: Option<String>, // .05 OUTCOME
}
