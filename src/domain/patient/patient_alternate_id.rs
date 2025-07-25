// Domain entity for Patient Alternate ID, modeled after VistA/MUMPS File #2.1
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatientAlternateId {
    pub patient_id: u32,             // .01 PATIENT (pointer)
    pub id_type: String,             // .02 ID TYPE
    pub id_value: String,            // .03 ID VALUE
}
