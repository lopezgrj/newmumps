// Domain entity for Clinic Patient Feedback (custom)
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClinicPatientFeedback {
    pub id: u32, // .01 FEEDBACK ID
    pub clinic_id: u32, // .02 CLINIC (pointer)
    pub patient_id: u32, // .03 PATIENT (pointer)
    pub feedback_date: String, // .04 FEEDBACK DATE
    pub rating: u8, // .05 RATING (1-5)
    pub comments: Option<String>, // .06 COMMENTS
}
