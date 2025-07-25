// Domain entity for Patient Sensitive Record Access subfile (VistA/MUMPS File #2, SENSITIVE RECORD ACCESS multiple)
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatientSensitiveRecordAccess {
    pub access_date: String, // .01 ACCESS DATE
    pub accessed_by: u32, // .02 ACCESSED BY (pointer)
    pub reason: Option<String>, // .03 REASON
}
