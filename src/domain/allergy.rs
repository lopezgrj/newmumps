// Domain entity for Allergy, modeled after VistA/MUMPS File #120.8
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Allergy {
    pub patient_id: u32,             // .01 PATIENT (pointer)
    pub reaction: String,            // 1 REACTION
    pub observed_date: Option<String>, // 2 OBSERVED DATE
    pub severity: Option<String>,    // 3 SEVERITY
    pub status: Option<String>,      // 4 STATUS
}
