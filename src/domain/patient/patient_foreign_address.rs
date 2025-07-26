// Domain entity for Patient Foreign Address subfile (VistA/MUMPS File #2, FOREIGN ADDRESS multiple)
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatientForeignAddress {
    pub address: String, // .01 FOREIGN ADDRESS
    pub country: String, // .02 COUNTRY
    pub start_date: Option<String>, // .03 START DATE
    pub end_date: Option<String>, // .04 END DATE
}
