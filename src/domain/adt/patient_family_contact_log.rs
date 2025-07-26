// Domain entity for Patient Family Contact Log (custom)
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatientFamilyContactLog {
    pub id: u32, // .01 LOG ID
    pub patient_id: u32, // .02 PATIENT (pointer)
    pub contact_date: String, // .03 CONTACT DATE
    pub family_member: String, // .04 FAMILY MEMBER
    pub notes: Option<String>, // .05 NOTES
}
