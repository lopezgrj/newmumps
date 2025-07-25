// Domain entity for Appointment Request, modeled after VistA/MUMPS File #409.61
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppointmentRequest {
    pub patient_id: u32,             // .01 PATIENT (pointer)
    pub request_date: String,        // .02 REQUEST DATE
    pub clinic_id: Option<u32>,      // .03 CLINIC (pointer)
    pub status: Option<String>,      // .04 STATUS
    pub reason: Option<String>,      // .05 REASON
}
