// Domain entity for Patient Custom Flag subfile (VistA/MUMPS File #2, CUSTOM FLAG multiple)
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatientCustomFlag {
    pub flag: String, // .01 FLAG
    pub date_set: Option<String>, // .02 DATE SET
    pub set_by: Option<String>, // .03 SET BY
    pub notes: Option<String>, // .04 NOTES
}
