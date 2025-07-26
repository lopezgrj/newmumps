// Domain entity for Patient Research Study Participation subfile (VistA/MUMPS File #2, RESEARCH STUDY PARTICIPATION multiple)
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatientResearchStudyParticipation {
    pub study_name: String, // .01 STUDY NAME
    pub enrollment_date: Option<String>, // .02 ENROLLMENT DATE
    pub completion_date: Option<String>, // .03 COMPLETION DATE
    pub status: Option<String>, // .04 STATUS
}
