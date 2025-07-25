// Domain entity for Patient Preferred Name History subfile (VistA/MUMPS File #2, PREFERRED NAME HISTORY multiple)
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatientPreferredNameHistory {
    pub preferred_name: String, // .01 PREFERRED NAME
    pub start_date: Option<String>, // .02 START DATE
    pub end_date: Option<String>, // .03 END DATE
}
