// Domain entity for Patient Spouse, modeled after VistA/MUMPS File #2.05
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatientSpouse {
    pub patient_id: u32,             // .01 PATIENT (pointer)
    pub name: String,                // .02 NAME
    pub phone: Option<String>,       // .03 PHONE
    pub address: Option<String>,     // .04 ADDRESS
}
