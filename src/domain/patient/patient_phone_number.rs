// Subfile for Patient Phone Numbers (#2.01)
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatientPhoneNumber {
    pub patient_id: u32,         // Parent patient
    pub phone_type: String,      // e.g., "Home", "Work", "Mobile"
    pub number: String,
}
