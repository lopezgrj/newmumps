// Domain entity for Clinic Session (VistA/MUMPS File #44.03)
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClinicSession {
    pub id: u32, // .01 SESSION ID
    pub clinic_id: u32, // .02 CLINIC (pointer)
    pub session_date: String, // .03 SESSION DATE
    pub start_time: String, // .04 START TIME
    pub end_time: String, // .05 END TIME
}
