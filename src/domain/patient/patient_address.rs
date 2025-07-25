// Subfile for Patient Address (#2.07)
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatientAddress {
    pub patient_id: u32,
    pub address_type: String,    // e.g., "Home", "Mailing"
    pub address: String,
    pub city: Option<String>,
    pub state: Option<String>,
    pub zip: Option<String>,
}
