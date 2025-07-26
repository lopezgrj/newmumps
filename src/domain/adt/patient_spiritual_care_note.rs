// Domain entity for Patient Spiritual Care Note (custom)
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatientSpiritualCareNote {
    pub id: u32, // .01 NOTE ID
    pub patient_id: u32, // .02 PATIENT (pointer)
    pub note_date: String, // .03 NOTE DATE
    pub content: String, // .04 CONTENT
    pub chaplain_id: Option<u32>, // .05 CHAPLAIN (pointer)
}
