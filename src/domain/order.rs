// Domain entity for Order, modeled after VistA/MUMPS File #100
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Order {
    pub patient_id: u32,             // .01 PATIENT (pointer)
    pub order_date: String,          // 1 ORDER DATE
    pub order_text: String,          // 2 ORDER TEXT
    pub provider_id: Option<u32>,    // 3 PROVIDER (pointer)
    pub status: Option<String>,      // 5 STATUS
}
