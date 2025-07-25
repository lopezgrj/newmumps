// Subfile for Patient Email (#2.08)
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatientEmail {
    pub patient_id: u32,
    pub email: String,
    pub email_type: Option<String>, // e.g., "Personal", "Work"
}
