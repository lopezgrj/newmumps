// Domain entity for Patient Employment subfile (VistA/MUMPS File #2, EMPLOYMENT multiple)
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatientEmployment {
    pub employer_name: String, // .01 EMPLOYER NAME
    pub start_date: Option<String>, // .02 START DATE
    pub end_date: Option<String>, // .03 END DATE
    pub position: Option<String>, // .04 POSITION
}
