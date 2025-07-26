// Domain entity for Clinic Patient Instructions (VistA/MUMPS File #44.12)
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClinicPatientInstruction {
    pub id: u32, // .01 INSTRUCTION ID
    pub clinic_id: u32, // .02 CLINIC (pointer)
    pub instruction: String, // .03 INSTRUCTION
}
