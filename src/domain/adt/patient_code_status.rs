// Domain entity for Patient Code Status (custom)
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatientCodeStatus {
    pub id: u32, // .01 STATUS ID
    pub patient_id: u32, // .02 PATIENT (pointer)
    pub code_status: String, // .03 CODE STATUS (e.g., DNR, Full Code)
    pub status_date: String, // .04 STATUS DATE
    pub notes: Option<String>, // .05 NOTES
}
