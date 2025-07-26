// Domain entity for Clinic Interpreter Request (custom)
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClinicInterpreterRequest {
    pub id: u32, // .01 REQUEST ID
    pub clinic_id: u32, // .02 CLINIC (pointer)
    pub patient_id: u32, // .03 PATIENT (pointer)
    pub language: String, // .04 LANGUAGE
    pub request_date: String, // .05 REQUEST DATE
    pub status: Option<String>, // .06 STATUS
}
