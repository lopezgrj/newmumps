// Domain entity for Patient Confidential Communication Preference subfile (VistA/MUMPS File #2, CONFIDENTIAL COMMUNICATION PREFERENCE multiple)
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatientConfidentialCommunicationPreference {
    pub preference: String, // .01 PREFERENCE
    pub start_date: Option<String>, // .02 START DATE
    pub end_date: Option<String>, // .03 END DATE
}
