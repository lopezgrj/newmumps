// Domain entity for Order Change Log (custom)
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderChangeLog {
    pub id: u32, // .01 LOG ID
    pub order_id: u32, // .02 ORDER (pointer)
    pub change_type: String, // .03 CHANGE TYPE
    pub change_date: String, // .04 CHANGE DATE
    pub user_id: Option<u32>, // .05 USER (pointer)
    pub notes: Option<String>, // .06 NOTES
}
