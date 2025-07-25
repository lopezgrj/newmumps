// Domain entity for Appointment Request Comment, modeled after VistA/MUMPS File #409.86
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppointmentRequestComment {
    pub request_id: u32,             // .01 REQUEST (pointer)
    pub comment: String,             // .02 COMMENT
    pub entered_by: Option<u32>,     // .03 ENTERED BY (pointer)
    pub entered_date: Option<String>,// .04 ENTERED DATE
}
