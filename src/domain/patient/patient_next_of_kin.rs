// Domain entity for Patient Next of Kin, modeled after VistA/MUMPS File #2.03
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatientNextOfKin {
    pub patient_id: u32,             // .01 PATIENT (pointer)
    pub name: String,                // .02 NAME
    pub relationship: Option<String>,// .03 RELATIONSHIP
    pub phone: Option<String>,       // .04 PHONE
    pub address: Option<String>,     // .05 ADDRESS
}
