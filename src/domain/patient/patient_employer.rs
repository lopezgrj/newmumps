// Domain entity for Patient Employer, modeled after VistA/MUMPS File #2.07
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatientEmployer {
    pub patient_id: u32,             // .01 PATIENT (pointer)
    pub employer_name: String,       // .02 EMPLOYER NAME
    pub phone: Option<String>,       // .03 PHONE
    pub address: Option<String>,     // .04 ADDRESS
}
