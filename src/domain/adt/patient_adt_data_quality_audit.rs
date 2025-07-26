// Domain entity for Patient ADT Data Quality Audit (custom)
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatientAdtDataQualityAudit {
    pub id: u32, // .01 AUDIT ID
    pub patient_id: u32, // .02 PATIENT (pointer)
    pub audit_date: String, // .03 AUDIT DATE
    pub field_audited: String, // .04 FIELD AUDITED
    pub result: String, // .05 RESULT
    pub user_id: Option<u32>, // .06 USER (pointer)
}
