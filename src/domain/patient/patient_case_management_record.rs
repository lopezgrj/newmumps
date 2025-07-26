// Domain entity for Patient Case Management Record subfile (VistA/MUMPS File #2, CASE MANAGEMENT RECORD multiple)
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatientCaseManagementRecord {
    pub case_id: String, // .01 CASE ID
    pub manager: Option<String>, // .02 MANAGER
    pub open_date: Option<String>, // .03 OPEN DATE
    pub close_date: Option<String>, // .04 CLOSE DATE
    pub status: Option<String>, // .05 STATUS
}
