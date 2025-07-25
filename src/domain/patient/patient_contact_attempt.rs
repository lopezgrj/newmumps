// Domain entity for Patient Contact Attempt subfile (VistA/MUMPS File #2, CONTACT ATTEMPT multiple)
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatientContactAttempt {
    pub attempt_date: String, // .01 ATTEMPT DATE
    pub method: Option<String>, // .02 METHOD (phone, mail, etc.)
    pub result: Option<String>, // .03 RESULT
    pub attempted_by: Option<u32>, // .04 ATTEMPTED BY (pointer)
}
