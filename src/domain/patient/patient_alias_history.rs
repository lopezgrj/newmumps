// Domain entity for Patient Alias History, modeled after VistA/MUMPS File #2.11
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatientAliasHistory {
    pub patient_id: u32,             // .01 PATIENT (pointer)
    pub alias: String,               // .02 ALIAS
    pub change_date: String,         // .03 CHANGE DATE
}
