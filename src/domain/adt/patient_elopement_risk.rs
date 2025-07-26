// Domain entity for Patient Elopement Risk (custom)
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatientElopementRisk {
    pub id: u32, // .01 RISK ID
    pub patient_id: u32, // .02 PATIENT (pointer)
    pub risk_date: String, // .03 RISK DATE
    pub risk_level: String, // .04 RISK LEVEL
    pub notes: Option<String>, // .05 NOTES
}
