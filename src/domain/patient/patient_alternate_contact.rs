// Domain entity for Patient Alternate Contact subfile (VistA/MUMPS File #2, ALTERNATE CONTACT multiple)
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatientAlternateContact {
    pub name: String, // .01 NAME
    pub relationship: Option<String>, // .02 RELATIONSHIP
    pub phone: Option<String>, // .03 PHONE
    pub address: Option<String>, // .04 ADDRESS
}
