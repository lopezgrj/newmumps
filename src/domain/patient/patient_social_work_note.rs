// Domain entity for Patient Social Work Note subfile (VistA/MUMPS File #2, SOCIAL WORK NOTE multiple)
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatientSocialWorkNote {
    pub note_date: String, // .01 NOTE DATE
    pub author: Option<String>, // .02 AUTHOR
    pub content: String, // .03 CONTENT
}
