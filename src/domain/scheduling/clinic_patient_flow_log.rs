// Domain entity for Clinic Patient Flow Log (VistA/MUMPS File #44.19)
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClinicPatientFlowLog {
    pub id: u32, // .01 LOG ID
    pub clinic_id: u32, // .02 CLINIC (pointer)
    pub patient_id: u32, // .03 PATIENT (pointer)
    pub flow_date: String, // .04 FLOW DATE
    pub status: Option<String>, // .05 STATUS
}
