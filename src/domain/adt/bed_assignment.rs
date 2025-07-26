// Domain entity for Bed Assignment (VistA/MUMPS File #405.4)
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BedAssignment {
    pub id: u32, // .01 ASSIGNMENT ID
    pub patient_id: u32, // .02 PATIENT (pointer)
    pub bed_id: u32, // .03 BED (pointer)
    pub assignment_date: String, // .04 ASSIGNMENT DATE
    pub discharge_date: Option<String>, // .05 DISCHARGE DATE
}
