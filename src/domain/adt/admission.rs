// Domain entity for Admission (VistA/MUMPS File #405)
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Admission {
    pub id: u32, // .01 ADMISSION ID
    pub patient_id: u32, // .02 PATIENT (pointer)
    pub admission_date: String, // .03 ADMISSION DATE
    pub admitting_location: Option<String>, // .04 ADMITTING LOCATION
    pub admitting_provider: Option<u32>, // .05 ADMITTING PROVIDER (pointer)
    pub admission_type: Option<String>, // .06 ADMISSION TYPE
}
