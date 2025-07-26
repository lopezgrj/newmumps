// Domain entity for Clinic Patient Notification (VistA/MUMPS File #409.86)
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClinicPatientNotification {
    pub id: u32, // .01 NOTIFICATION ID
    pub patient_id: u32, // .02 PATIENT (pointer)
    pub clinic_id: u32, // .03 CLINIC (pointer)
    pub notification_date: String, // .04 NOTIFICATION DATE
    pub method: Option<String>, // .05 METHOD
    pub status: Option<String>, // .06 STATUS
}
