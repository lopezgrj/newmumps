// Domain entity for Patient Movement, modeled after VistA/MUMPS File #405
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatientMovement {
    pub patient_id: u32,             // .01 PATIENT (pointer to Patient)
    pub movement_date_time: String,  // .02 MOVEMENT DATE/TIME
    pub movement_type: String,       // .03 TYPE OF MOVEMENT (Admission, Transfer, etc.)
    pub from_location: Option<String>, // .04 FROM WARD/LOCATION
    pub to_location: Option<String>, // .05 TO WARD/LOCATION
    pub attending_physician: Option<String>, // .06 ATTENDING PHYSICIAN
    pub specialty: Option<String>,   // .08 SPECIALTY
    pub admitting_diagnosis: Option<String>, // .07 ADMITTING DIAGNOSIS
}
