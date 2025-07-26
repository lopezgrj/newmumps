// Domain entity for Patient Care Team Assignment subfile (VistA/MUMPS File #2, CARE TEAM ASSIGNMENT multiple)
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatientCareTeamAssignment {
    pub team_name: String, // .01 TEAM NAME
    pub start_date: Option<String>, // .02 START DATE
    pub end_date: Option<String>, // .03 END DATE
    pub role: Option<String>, // .04 ROLE
}
