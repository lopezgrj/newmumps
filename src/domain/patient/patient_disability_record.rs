// Domain entity for Patient Disability Record subfile (VistA/MUMPS File #2, DISABILITY RECORD multiple)
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatientDisabilityRecord {
    pub disability: String, // .01 DISABILITY
    pub percent: Option<u8>, // .02 PERCENT
    pub service_connected: Option<bool>, // .03 SERVICE CONNECTED
}
