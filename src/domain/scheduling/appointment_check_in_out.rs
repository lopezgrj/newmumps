// Domain entity for Appointment Check-In/Out, modeled after VistA/MUMPS File #409.81
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppointmentCheckInOut {
    pub appointment_id: u32,         // .01 APPOINTMENT (pointer)
    pub check_in_time: Option<String>, // .02 CHECK-IN TIME
    pub check_out_time: Option<String>, // .03 CHECK-OUT TIME
    pub user_id: Option<u32>,        // .04 USER (pointer)
}
