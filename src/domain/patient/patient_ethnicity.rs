// Domain entity for Patient Ethnicity, modeled after VistA/MUMPS File #27.18
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatientEthnicity {
    pub patient_id: u32,             // .01 PATIENT (pointer)
    pub ethnicity: String,           // .02 ETHNICITY
    pub entered_date: Option<String>,// .03 ENTERED DATE
}
