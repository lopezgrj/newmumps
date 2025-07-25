// Domain entity for Patient POW Status, modeled after VistA/MUMPS File #2.09
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatientPOWStatus {
    pub patient_id: u32,             // .01 PATIENT (pointer)
    pub pow_status: bool,            // .02 POW STATUS
    pub capture_date: Option<String>,// .03 CAPTURE DATE
    pub release_date: Option<String>,// .04 RELEASE DATE
}
