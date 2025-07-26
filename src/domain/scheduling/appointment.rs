// Domain entity for Appointment (VistA/MUMPS File #409.84)
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Appointment {
    pub id: u32, // .01 APPOINTMENT ID
    pub patient_id: u32, // .02 PATIENT (pointer)
    pub clinic_id: u32, // .03 CLINIC (pointer)
    pub start_time: String, // .04 START TIME
    pub end_time: Option<String>, // .05 END TIME
    pub status: Option<String>, // .06 STATUS
}
