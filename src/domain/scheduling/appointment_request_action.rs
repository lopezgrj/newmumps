// Domain entity for Appointment Request Action, modeled after VistA/MUMPS File #409.88
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppointmentRequestAction {
    pub request_id: u32,             // .01 REQUEST (pointer)
    pub action_type_id: u32,         // .02 ACTION TYPE (pointer)
    pub action_date: String,         // .03 ACTION DATE
    pub performed_by: Option<u32>,   // .04 PERFORMED BY (pointer)
    pub comment: Option<String>,     // .05 COMMENT
}
