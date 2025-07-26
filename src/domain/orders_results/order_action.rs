// Domain entity for Order Action (VistA/MUMPS File #100.02)
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderAction {
    pub id: u32, // .01 ACTION ID
    pub order_id: u32, // .02 ORDER (pointer)
    pub action_type: String, // .03 ACTION TYPE
    pub action_date: String, // .04 ACTION DATE
    pub user_id: Option<u32>, // .05 USER (pointer)
}
