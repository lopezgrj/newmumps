// Domain entity for Patient Movement History, modeled after VistA/MUMPS File #45.7
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatientMovementHistory {
    pub patient_id: u32,             // .01 PATIENT (pointer)
    pub movement_date: String,       // .02 MOVEMENT DATE
    pub movement_type: String,       // .03 MOVEMENT TYPE
    pub from_location: Option<String>, // .04 FROM LOCATION
    pub to_location: Option<String>, // .05 TO LOCATION
}
