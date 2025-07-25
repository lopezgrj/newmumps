// Domain entity for Appointment Check-In/Out Status History, modeled after VistA/MUMPS File #409.83
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppointmentCheckInOutStatusHistory {
    pub check_in_out_id: u32,        // .01 CHECK-IN/OUT (pointer)
    pub status: String,              // .02 STATUS
    pub status_date: String,         // .03 STATUS DATE
    pub changed_by: Option<u32>,     // .04 CHANGED BY (pointer)
}
