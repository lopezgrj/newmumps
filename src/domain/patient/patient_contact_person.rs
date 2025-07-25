// Domain entity for Patient Contact Person, modeled after VistA/MUMPS File #2.06
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatientContactPerson {
    pub patient_id: u32,             // .01 PATIENT (pointer)
    pub name: String,                // .02 NAME
    pub relationship: Option<String>,// .03 RELATIONSHIP
    pub phone: Option<String>,       // .04 PHONE
    pub address: Option<String>,     // .05 ADDRESS
}
