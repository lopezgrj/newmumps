// Domain entity for Patient Fall Risk Assessment (custom)
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatientFallRiskAssessment {
    pub id: u32, // .01 ASSESSMENT ID
    pub patient_id: u32, // .02 PATIENT (pointer)
    pub assessment_date: String, // .03 ASSESSMENT DATE
    pub risk_level: String, // .04 RISK LEVEL
    pub notes: Option<String>, // .05 NOTES
}
