// Domain entity for Patient Death Data, modeled after VistA/MUMPS File #2.15
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatientDeathData {
    pub patient_id: u32,             // .01 PATIENT (pointer)
    pub date_of_death: String,       // .02 DATE OF DEATH
    pub cause_of_death: Option<String>, // .03 CAUSE OF DEATH
    pub place_of_death: Option<String>, // .04 PLACE OF DEATH
}
