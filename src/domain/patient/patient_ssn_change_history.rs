// Domain entity for Patient SSN Change History, modeled after VistA/MUMPS File #2.14
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatientSsnChangeHistory {
    pub patient_id: u32,             // .01 PATIENT (pointer)
    pub old_ssn: String,             // .02 OLD SSN
    pub new_ssn: String,             // .03 NEW SSN
    pub change_date: String,         // .04 CHANGE DATE
}
