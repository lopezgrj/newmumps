// Domain entity for Patient Preferred Language History subfile (VistA/MUMPS File #2, PREFERRED LANGUAGE HISTORY multiple)
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatientPreferredLanguageHistory {
    pub language: String, // .01 LANGUAGE
    pub start_date: Option<String>, // .02 START DATE
    pub end_date: Option<String>, // .03 END DATE
}
