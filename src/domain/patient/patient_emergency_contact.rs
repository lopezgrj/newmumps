// Domain entity for Patient Emergency Contact, modeled after VistA/MUMPS File #2.02
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatientEmergencyContact {
    pub patient_id: u32,             // .01 PATIENT (pointer)
    pub contact_name: String,        // .02 CONTACT NAME
    pub relationship: Option<String>,// .03 RELATIONSHIP
    pub phone: Option<String>,       // .04 PHONE
    pub address: Option<String>,     // .05 ADDRESS
}
