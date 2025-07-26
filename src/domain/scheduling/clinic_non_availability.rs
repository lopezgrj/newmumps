// Domain entity for Clinic Non-Availability (VistA/MUMPS File #44.04)
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClinicNonAvailability {
    pub id: u32, // .01 NON-AVAILABILITY ID
    pub clinic_id: u32, // .02 CLINIC (pointer)
    pub start_date: String, // .03 START DATE
    pub end_date: String, // .04 END DATE
    pub reason: Option<String>, // .05 REASON
}
