// Domain entity for Clinic Special Instructions (VistA/MUMPS File #44.07)
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClinicSpecialInstructions {
    pub id: u32, // .01 INSTRUCTION ID
    pub clinic_id: u32, // .02 CLINIC (pointer)
    pub instruction: String, // .03 INSTRUCTION
}
