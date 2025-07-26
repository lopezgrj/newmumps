// Domain entity for Clinic Access Restrictions (VistA/MUMPS File #44.11)
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClinicAccessRestriction {
    pub id: u32, // .01 RESTRICTION ID
    pub clinic_id: u32, // .02 CLINIC (pointer)
    pub restriction_type: String, // .03 RESTRICTION TYPE
    pub start_date: String, // .04 START DATE
    pub end_date: Option<String>, // .05 END DATE
    pub notes: Option<String>, // .06 NOTES
}
