// Domain entity for Patient Marital Status subfile (VistA/MUMPS File #2, MARITAL STATUS multiple)
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatientMaritalStatus {
    pub marital_status: String, // .01 MARITAL STATUS
    pub effective_date: Option<String>, // .02 EFFECTIVE DATE
}
