// Domain entity for Patient Prisoner Information subfile (VistA/MUMPS File #2, PRISONER INFO multiple)
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatientPrisonerInfo {
    pub facility: String, // .01 FACILITY
    pub incarceration_date: Option<String>, // .02 INCARCERATION DATE
    pub release_date: Option<String>, // .03 RELEASE DATE
}
