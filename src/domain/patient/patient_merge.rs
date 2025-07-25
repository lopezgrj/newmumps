// Domain entity for Patient Merge, modeled after VistA/MUMPS File #2.98
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatientMerge {
    pub from_patient_id: u32,        // .01 FROM PATIENT (pointer)
    pub to_patient_id: u32,          // .02 TO PATIENT (pointer)
    pub merge_date: String,          // .03 MERGE DATE
    pub merged_by: Option<u32>,      // .04 MERGED BY (pointer)
}
