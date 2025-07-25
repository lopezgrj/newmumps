// Domain entity for Patient Insurance Data, modeled after VistA/MUMPS File #47
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatientInsuranceData {
    pub patient_id: u32,             // .01 PATIENT (pointer)
    pub policy_id: u32,              // .02 POLICY (pointer)
    pub coverage_type: Option<String>, // .03 COVERAGE TYPE
    pub status: Option<String>,      // .04 STATUS
}
