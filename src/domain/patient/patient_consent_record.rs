// Domain entity for Patient Consent Record subfile (VistA/MUMPS File #2, CONSENT RECORD multiple)
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatientConsentRecord {
    pub consent_type: String, // .01 CONSENT TYPE
    pub date_given: Option<String>, // .02 DATE GIVEN
    pub details: Option<String>, // .03 DETAILS
}
