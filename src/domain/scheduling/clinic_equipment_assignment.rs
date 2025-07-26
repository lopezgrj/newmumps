// Domain entity for Clinic Equipment Assignment (VistA/MUMPS File #44.14)
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClinicEquipmentAssignment {
    pub id: u32, // .01 ASSIGNMENT ID
    pub clinic_id: u32, // .02 CLINIC (pointer)
    pub equipment: String, // .03 EQUIPMENT
    pub start_date: String, // .04 START DATE
    pub end_date: Option<String>, // .05 END DATE
}
