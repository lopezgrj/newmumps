// Domain entity for Clinic Room Assignment (VistA/MUMPS File #44.13)
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClinicRoomAssignment {
    pub id: u32, // .01 ASSIGNMENT ID
    pub clinic_id: u32, // .02 CLINIC (pointer)
    pub room: String, // .03 ROOM
    pub start_date: String, // .04 START DATE
    pub end_date: Option<String>, // .05 END DATE
}
