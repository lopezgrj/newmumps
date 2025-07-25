// Domain entity for Patient Deceased Information subfile (VistA/MUMPS File #2, DECEASED INFO multiple)
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatientDeceasedInfo {
    pub date_of_death: String, // .01 DATE OF DEATH
    pub cause_of_death: Option<String>, // .02 CAUSE OF DEATH
    pub place_of_death: Option<String>, // .03 PLACE OF DEATH
}
