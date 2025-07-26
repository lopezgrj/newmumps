// Domain entity for Patient Temporary Address subfile (VistA/MUMPS File #2, TEMPORARY ADDRESS multiple)
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatientTemporaryAddress {
    pub address: String, // .01 TEMPORARY ADDRESS
    pub start_date: Option<String>, // .02 START DATE
    pub end_date: Option<String>, // .03 END DATE
    pub reason: Option<String>, // .04 REASON
}
