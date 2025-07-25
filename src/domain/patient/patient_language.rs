// Domain entity for Patient Language subfile (VistA/MUMPS File #2, LANGUAGE multiple)
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatientLanguage {
    pub language_id: u32, // .01 LANGUAGE (pointer)
    pub primary: bool, // .02 PRIMARY LANGUAGE
}
