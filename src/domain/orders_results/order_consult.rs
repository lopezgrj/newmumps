// Domain entity for Order Consult (VistA/MUMPS File #123)
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderConsult {
    pub id: u32, // .01 CONSULT ID
    pub order_id: u32, // .02 ORDER (pointer)
    pub service: String, // .03 SERVICE
    pub request_date: String, // .04 REQUEST DATE
    pub status: String, // .05 STATUS
}
