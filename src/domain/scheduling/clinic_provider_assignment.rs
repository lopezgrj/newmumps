// Domain entity for Clinic Provider Assignment (VistA/MUMPS File #44.08)
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClinicProviderAssignment {
    pub id: u32, // .01 ASSIGNMENT ID
    pub clinic_id: u32, // .02 CLINIC (pointer)
    pub provider_id: u32, // .03 PROVIDER (pointer)
    pub start_date: String, // .04 START DATE
    pub end_date: Option<String>, // .05 END DATE
}
