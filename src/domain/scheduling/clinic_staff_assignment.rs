// Domain entity for Clinic Staff Assignment (custom)
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClinicStaffAssignment {
    pub id: u32, // .01 ASSIGNMENT ID
    pub clinic_id: u32, // .02 CLINIC (pointer)
    pub staff_id: u32, // .03 STAFF (pointer)
    pub role: String, // .04 ROLE
    pub start_date: String, // .05 START DATE
    pub end_date: Option<String>, // .06 END DATE
}
