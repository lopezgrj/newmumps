// Domain entity for Outpatient Encounter (VistA/MUMPS File #409)
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OutpatientEncounter {
    pub id: u32, // .01 ENCOUNTER ID
    pub patient_id: u32, // .02 PATIENT (pointer)
    pub clinic_id: u32, // .03 CLINIC (pointer)
    pub encounter_date: String, // .04 ENCOUNTER DATE
    pub status: Option<String>, // .05 STATUS
}
