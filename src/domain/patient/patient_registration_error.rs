// Domain entity for Patient Registration Error, modeled after VistA/MUMPS File #2.99
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatientRegistrationError {
    pub patient_id: u32,             // .01 PATIENT (pointer)
    pub error_date: String,          // .02 ERROR DATE
    pub error_type: String,          // .03 ERROR TYPE
    pub description: Option<String>, // .04 DESCRIPTION
}
