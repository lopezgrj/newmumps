// Domain entity for Patient Deceased Contact subfile (VistA/MUMPS File #2, DECEASED CONTACT multiple)
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatientDeceasedContact {
    pub name: String, // .01 NAME
    pub relationship: Option<String>, // .02 RELATIONSHIP
    pub phone: Option<String>, // .03 PHONE
    pub address: Option<String>, // .04 ADDRESS
}
