// Domain entity for Appointment Check-In/Out Comment, modeled after VistA/MUMPS File #409.82
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppointmentCheckInOutComment {
    pub check_in_out_id: u32,        // .01 CHECK-IN/OUT (pointer)
    pub comment: String,             // .02 COMMENT
    pub entered_by: Option<u32>,     // .03 ENTERED BY (pointer)
    pub entered_date: Option<String>,// .04 ENTERED DATE
}
