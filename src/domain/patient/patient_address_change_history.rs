// Domain entity for Patient Address Change History, modeled after VistA/MUMPS File #2.12
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatientAddressChangeHistory {
    pub patient_id: u32,             // .01 PATIENT (pointer)
    pub old_address: String,         // .02 OLD ADDRESS
    pub new_address: String,         // .03 NEW ADDRESS
    pub change_date: String,         // .04 CHANGE DATE
}
