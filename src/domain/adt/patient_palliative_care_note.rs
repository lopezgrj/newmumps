// Domain entity for Patient Palliative Care Note (custom)
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatientPalliativeCareNote {
    pub id: u32, // .01 NOTE ID
    pub patient_id: u32, // .02 PATIENT (pointer)
    pub note_date: String, // .03 NOTE DATE
    pub content: String, // .04 CONTENT
    pub provider_id: Option<u32>, // .05 PROVIDER (pointer)
}
