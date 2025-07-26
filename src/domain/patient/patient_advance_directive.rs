// Domain entity for Patient Advance Directive subfile (VistA/MUMPS File #2, ADVANCE DIRECTIVE multiple)
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatientAdvanceDirective {
    pub directive_type: String, // .01 DIRECTIVE TYPE
    pub date_recorded: Option<String>, // .02 DATE RECORDED
    pub details: Option<String>, // .03 DETAILS
}
