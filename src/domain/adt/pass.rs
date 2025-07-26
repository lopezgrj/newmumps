// Domain entity for Pass (VistA/MUMPS File #405.6)
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pass {
    pub id: u32, // .01 PASS ID
    pub patient_id: u32, // .02 PATIENT (pointer)
    pub pass_date: String, // .03 PASS DATE
    pub return_date: Option<String>, // .04 RETURN DATE
    pub reason: Option<String>, // .05 REASON
}
