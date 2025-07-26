// Domain entity for Patient Home Health Visit subfile (VistA/MUMPS File #2, HOME HEALTH VISIT multiple)
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatientHomeHealthVisit {
    pub visit_date: String, // .01 VISIT DATE
    pub provider: Option<String>, // .02 PROVIDER
    pub notes: Option<String>, // .03 NOTES
}
