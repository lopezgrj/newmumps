// Domain entity for Clinic Telehealth Session (VistA/MUMPS File #44.15)
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClinicTelehealthSession {
    pub id: u32, // .01 SESSION ID
    pub clinic_id: u32, // .02 CLINIC (pointer)
    pub session_date: String, // .03 SESSION DATE
    pub modality: String, // .04 MODALITY
    pub notes: Option<String>, // .05 NOTES
}
