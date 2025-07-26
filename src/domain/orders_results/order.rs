// Domain entity for Order (VistA/MUMPS File #100)
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Order {
    pub id: u32, // .01 ORDER ID
    pub patient_id: u32, // .02 PATIENT (pointer)
    pub order_date: String, // .03 ORDER DATE
    pub provider_id: u32, // .04 PROVIDER (pointer)
    pub order_text: String, // .05 ORDER TEXT
    pub status: String, // .06 STATUS
}
