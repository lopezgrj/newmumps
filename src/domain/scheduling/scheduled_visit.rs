// Domain entity for Scheduled Visit, modeled after VistA/MUMPS File #409.5
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScheduledVisit {
    pub patient_id: u32,             // .01 PATIENT (pointer)
    pub clinic_id: u32,              // .02 CLINIC (pointer)
    pub appointment_date: String,    // .03 APPOINTMENT DATE
    pub appointment_type: Option<String>, // .04 APPOINTMENT TYPE
    pub status: Option<String>,      // .05 STATUS
}
