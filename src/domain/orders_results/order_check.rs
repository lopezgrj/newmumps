// Domain entity for Order Check (VistA/MUMPS File #100.8)
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderCheck {
    pub id: u32, // .01 CHECK ID
    pub order_id: u32, // .02 ORDER (pointer)
    pub check_type: String, // .03 CHECK TYPE
    pub result: String, // .04 RESULT
    pub checked_by: Option<u32>, // .05 CHECKED BY (pointer)
    pub check_date: Option<String>, // .06 CHECK DATE
}
