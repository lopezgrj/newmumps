// Domain entity for Ward Stay (VistA/MUMPS File #42)
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WardStay {
    pub id: u32, // .01 STAY ID
    pub patient_id: u32, // .02 PATIENT (pointer)
    pub ward_id: u32, // .03 WARD (pointer)
    pub admit_date: String, // .04 ADMIT DATE
    pub discharge_date: Option<String>, // .05 DISCHARGE DATE
}
