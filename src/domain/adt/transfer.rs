// Domain entity for Transfer (VistA/MUMPS File #405, transfer movement)
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transfer {
    pub id: u32, // .01 TRANSFER ID
    pub patient_id: u32, // .02 PATIENT (pointer)
    pub transfer_date: String, // .03 TRANSFER DATE
    pub from_location: Option<String>, // .04 FROM LOCATION
    pub to_location: Option<String>, // .05 TO LOCATION
    pub transfer_type: Option<String>, // .06 TRANSFER TYPE
}
