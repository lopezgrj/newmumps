// Domain entity for Patient Comment subfile (VistA/MUMPS File #2, COMMENT multiple)
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatientComment {
    pub comment: String, // .01 COMMENT
    pub entered_by: Option<u32>, // .02 ENTERED BY (pointer)
    pub entry_date: Option<String>, // .03 DATE ENTERED
}
