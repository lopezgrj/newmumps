// Domain entity for Clinic No-Show Log (VistA/MUMPS File #44.10)
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClinicNoShowLog {
    pub id: u32, // .01 LOG ID
    pub clinic_id: u32, // .02 CLINIC (pointer)
    pub patient_id: u32, // .03 PATIENT (pointer)
    pub no_show_date: String, // .04 NO-SHOW DATE
    pub reason: Option<String>, // .05 REASON
}
