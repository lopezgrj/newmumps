// Domain entity for Leave of Absence (VistA/MUMPS File #405.5)
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LeaveOfAbsence {
    pub id: u32, // .01 LEAVE ID
    pub patient_id: u32, // .02 PATIENT (pointer)
    pub leave_date: String, // .03 LEAVE DATE
    pub return_date: Option<String>, // .04 RETURN DATE
    pub reason: Option<String>, // .05 REASON
}
