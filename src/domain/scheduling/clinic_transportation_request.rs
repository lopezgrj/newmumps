// Domain entity for Clinic Transportation Request (custom)
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClinicTransportationRequest {
    pub id: u32, // .01 REQUEST ID
    pub clinic_id: u32, // .02 CLINIC (pointer)
    pub patient_id: u32, // .03 PATIENT (pointer)
    pub request_date: String, // .04 REQUEST DATE
    pub status: Option<String>, // .05 STATUS
}
