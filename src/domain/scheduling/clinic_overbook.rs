// Domain entity for Clinic Overbook (VistA/MUMPS File #44.06)
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClinicOverbook {
    pub id: u32, // .01 OVERBOOK ID
    pub clinic_id: u32, // .02 CLINIC (pointer)
    pub overbook_date: String, // .03 OVERBOOK DATE
    pub reason: Option<String>, // .04 REASON
}
