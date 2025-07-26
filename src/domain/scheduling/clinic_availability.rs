// Domain entity for Clinic Availability (VistA/MUMPS File #44.01)
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClinicAvailability {
    pub clinic_id: u32, // .01 CLINIC (pointer)
    pub available_date: String, // .02 AVAILABLE DATE
    pub start_time: String, // .03 START TIME
    pub end_time: String, // .04 END TIME
    pub slots: u32, // .05 SLOTS
}
