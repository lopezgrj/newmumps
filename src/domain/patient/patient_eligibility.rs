// Domain entity for Patient Eligibility, modeled after VistA/MUMPS File #38.1
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatientEligibility {
    pub patient_id: u32,             // .01 PATIENT (pointer)
    pub eligibility_code: String,    // .02 ELIGIBILITY CODE
    pub effective_date: Option<String>, // .03 EFFECTIVE DATE
}
