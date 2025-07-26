// Domain entity for Clinic Wait List History (VistA/MUMPS File #409.34)
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClinicWaitListHistory {
    pub id: u32, // .01 HISTORY ID
    pub wait_list_id: u32, // .02 WAIT LIST (pointer)
    pub action: String, // .03 ACTION
    pub action_date: String, // .04 ACTION DATE
    pub user_id: Option<u32>, // .05 USER (pointer)
}
