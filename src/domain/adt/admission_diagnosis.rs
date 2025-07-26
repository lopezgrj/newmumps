// Domain entity for Admission Diagnosis (VistA/MUMPS File #405.1)
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdmissionDiagnosis {
    pub id: u32, // .01 DIAGNOSIS ID
    pub admission_id: u32, // .02 ADMISSION (pointer)
    pub diagnosis_code: String, // .03 DIAGNOSIS CODE (ICD)
    pub diagnosis_date: String, // .04 DIAGNOSIS DATE
    pub provider_id: Option<u32>, // .05 PROVIDER (pointer)
}
