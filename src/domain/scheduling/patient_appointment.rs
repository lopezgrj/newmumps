// Domain entity for Patient Appointment, modeled after VistA/MUMPS File #1907
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatientAppointment {
    pub patient_id: u32,             // .01 PATIENT (pointer)
    pub appointment_date: String,    // .02 APPOINTMENT DATE
    pub clinic_id: Option<u32>,      // .03 CLINIC (pointer)
    pub status: Option<String>,      // .04 STATUS
    pub type_: Option<String>,       // .05 TYPE
}
