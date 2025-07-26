// Domain entity for Patient Restraint/Seclusion Log (custom)
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatientRestraintLog {
    pub id: u32, // .01 LOG ID
    pub patient_id: u32, // .02 PATIENT (pointer)
    pub start_date: String, // .03 START DATE
    pub end_date: Option<String>, // .04 END DATE
    pub reason: String, // .05 REASON
    pub notes: Option<String>, // .06 NOTES
}
