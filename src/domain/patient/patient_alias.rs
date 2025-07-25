// Domain entity for Patient Alias, modeled after VistA/MUMPS File #2.001
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatientAlias {
    pub patient_id: u32,             // .01 PATIENT (pointer)
    pub alias: String,               // .02 ALIAS
}
