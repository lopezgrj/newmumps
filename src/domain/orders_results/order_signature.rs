// Domain entity for Order Signature (custom)
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderSignature {
    pub id: u32, // .01 SIGNATURE ID
    pub order_id: u32, // .02 ORDER (pointer)
    pub signed_by: u32, // .03 SIGNED BY (pointer)
    pub signature_date: String, // .04 SIGNATURE DATE
}
