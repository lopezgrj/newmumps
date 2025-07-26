// Domain entity for Clinic Letter (VistA/MUMPS File #407.5)
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClinicLetter {
    pub id: u32, // .01 LETTER ID
    pub clinic_id: u32, // .02 CLINIC (pointer)
    pub letter_type: String, // .03 LETTER TYPE
    pub content: String, // .04 CONTENT
}
