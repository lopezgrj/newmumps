// Domain entity for Patient Religion, modeled after VistA/MUMPS File #28
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatientReligion {
    pub patient_id: u32,             // .01 PATIENT (pointer)
    pub religion: String,            // .02 RELIGION
    pub entered_date: Option<String>,// .03 ENTERED DATE
}
