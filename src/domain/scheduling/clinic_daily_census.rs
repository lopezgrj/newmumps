// Domain entity for Clinic Daily Census (custom)
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClinicDailyCensus {
    pub id: u32, // .01 CENSUS ID
    pub clinic_id: u32, // .02 CLINIC (pointer)
    pub census_date: String, // .03 CENSUS DATE
    pub patient_count: u32, // .04 PATIENT COUNT
}
