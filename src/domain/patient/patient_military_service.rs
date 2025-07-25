// Domain entity for Patient Military Service, modeled after VistA/MUMPS File #2.08
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatientMilitaryService {
    pub patient_id: u32,             // .01 PATIENT (pointer)
    pub branch: String,              // .02 BRANCH
    pub entry_date: Option<String>,  // .03 ENTRY DATE
    pub separation_date: Option<String>, // .04 SEPARATION DATE
    pub discharge_type: Option<String>, // .05 DISCHARGE TYPE
}
