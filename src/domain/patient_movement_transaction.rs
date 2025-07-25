// Domain entity for Patient Movement Transaction, modeled after VistA/MUMPS File #45
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatientMovementTransaction {
    pub patient_id: u32,             // .01 PATIENT (pointer)
    pub transaction_date: String,    // .02 TRANSACTION DATE
    pub movement_type: String,       // .03 TYPE OF MOVEMENT
    pub from_location: Option<String>, // .04 FROM LOCATION
    pub to_location: Option<String>, // .05 TO LOCATION
    pub reason: Option<String>,      // .06 REASON
}
