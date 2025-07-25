// Domain entity for Appointment Request Status History, modeled after VistA/MUMPS File #409.87
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppointmentRequestStatusHistory {
    pub request_id: u32,             // .01 REQUEST (pointer)
    pub status: String,              // .02 STATUS
    pub status_date: String,         // .03 STATUS DATE
    pub changed_by: Option<u32>,     // .04 CHANGED BY (pointer)
}
