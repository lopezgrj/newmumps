// Domain entity for Patient Movement (VistA/MUMPS File #405, all movement types)
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatientMovement {
    pub id: u32, // .01 MOVEMENT ID
    pub patient_id: u32, // .02 PATIENT (pointer)
    pub movement_type: String, // .03 MOVEMENT TYPE (admission, transfer, discharge, etc.)
    pub movement_date: String, // .04 MOVEMENT DATE
    pub from_location: Option<String>, // .05 FROM LOCATION
    pub to_location: Option<String>, // .06 TO LOCATION
    pub provider_id: Option<u32>, // .07 PROVIDER (pointer)
}
