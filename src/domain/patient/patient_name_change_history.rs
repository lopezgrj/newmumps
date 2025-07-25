// Domain entity for Patient Name Change History, modeled after VistA/MUMPS File #2.13
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatientNameChangeHistory {
    pub patient_id: u32,             // .01 PATIENT (pointer)
    pub old_name: String,            // .02 OLD NAME
    pub new_name: String,            // .03 NEW NAME
    pub change_date: String,         // .04 CHANGE DATE
}
