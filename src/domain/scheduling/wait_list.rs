// Domain entity for Wait List (VistA/MUMPS File #409.3)
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WaitListEntry {
    pub id: u32, // .01 ENTRY ID
    pub patient_id: u32, // .02 PATIENT (pointer)
    pub clinic_id: Option<u32>, // .03 CLINIC (pointer)
    pub request_date: String, // .04 REQUEST DATE
    pub status: Option<String>, // .05 STATUS
}
