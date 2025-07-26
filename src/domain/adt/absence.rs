// Domain entity for Absence (VistA/MUMPS File #405.7)
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Absence {
    pub id: u32, // .01 ABSENCE ID
    pub patient_id: u32, // .02 PATIENT (pointer)
    pub absence_date: String, // .03 ABSENCE DATE
    pub return_date: Option<String>, // .04 RETURN DATE
    pub reason: Option<String>, // .05 REASON
}
