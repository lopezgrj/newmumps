// Domain entity for Result Verification (custom)
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResultVerification {
    pub id: u32, // .01 VERIFICATION ID
    pub result_id: u32, // .02 RESULT (pointer)
    pub verified_by: u32, // .03 VERIFIED BY (pointer)
    pub verification_date: String, // .04 VERIFICATION DATE
}
