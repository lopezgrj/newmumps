// Domain entity for Patient ADT Exception Report (custom)
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatientAdtExceptionReport {
    pub id: u32, // .01 REPORT ID
    pub patient_id: u32, // .02 PATIENT (pointer)
    pub report_date: String, // .03 REPORT DATE
    pub exception_type: String, // .04 EXCEPTION TYPE
    pub description: String, // .05 DESCRIPTION
    pub user_id: Option<u32>, // .06 USER (pointer)
}
