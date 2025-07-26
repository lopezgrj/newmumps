// Domain entity for Patient ADT Data Correction Log (custom)
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatientAdtDataCorrectionLog {
    pub id: u32, // .01 LOG ID
    pub patient_id: u32, // .02 PATIENT (pointer)
    pub correction_date: String, // .03 CORRECTION DATE
    pub field_corrected: String, // .04 FIELD CORRECTED
    pub old_value: String, // .05 OLD VALUE
    pub new_value: String, // .06 NEW VALUE
    pub user_id: Option<u32>, // .07 USER (pointer)
}
