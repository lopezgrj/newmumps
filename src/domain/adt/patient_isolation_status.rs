// Domain entity for Patient Isolation Status (custom)
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatientIsolationStatus {
    pub id: u32, // .01 STATUS ID
    pub patient_id: u32, // .02 PATIENT (pointer)
    pub isolation_type: String, // .03 ISOLATION TYPE
    pub start_date: String, // .04 START DATE
    pub end_date: Option<String>, // .05 END DATE
    pub notes: Option<String>, // .06 NOTES
}
