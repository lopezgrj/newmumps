// Domain entity for Clinic Add/Cancel Log (VistA/MUMPS File #44.05)
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClinicAddCancelLog {
    pub id: u32, // .01 LOG ID
    pub clinic_id: u32, // .02 CLINIC (pointer)
    pub action: String, // .03 ACTION (add/cancel)
    pub action_date: String, // .04 ACTION DATE
    pub user_id: Option<u32>, // .05 USER (pointer)
}
