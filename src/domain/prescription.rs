// Domain entity for Prescription, modeled after VistA/MUMPS File #52
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Prescription {
    pub patient_id: u32,             // .01 PATIENT (pointer)
    pub drug_id: u32,                // 6 DRUG (pointer)
    pub issue_date: String,          // 1 ISSUE DATE
    pub quantity: Option<u32>,       // 7 QUANTITY
    pub provider_id: Option<u32>,    // 4 PROVIDER (pointer)
    pub status: Option<String>,      // 100 STATUS
}
