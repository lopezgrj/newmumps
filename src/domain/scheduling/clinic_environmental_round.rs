// Domain entity for Clinic Environmental Round (custom)
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClinicEnvironmentalRound {
    pub id: u32, // .01 ROUND ID
    pub clinic_id: u32, // .02 CLINIC (pointer)
    pub round_date: String, // .03 ROUND DATE
    pub findings: Option<String>, // .04 FINDINGS
}
