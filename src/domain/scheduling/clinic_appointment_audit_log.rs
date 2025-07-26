// Domain entity for Clinic Appointment Audit Log (VistA/MUMPS File #44.17)
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClinicAppointmentAuditLog {
    pub id: u32, // .01 LOG ID
    pub appointment_id: u32, // .02 APPOINTMENT (pointer)
    pub action: String, // .03 ACTION
    pub action_date: String, // .04 ACTION DATE
    pub user_id: Option<u32>, // .05 USER (pointer)
}
