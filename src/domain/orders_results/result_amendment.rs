// Domain entity for Result Amendment (custom)
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResultAmendment {
    pub id: u32, // .01 AMENDMENT ID
    pub result_id: u32, // .02 RESULT (pointer)
    pub amendment_date: String, // .03 AMENDMENT DATE
    pub amended_by: u32, // .04 AMENDED BY (pointer)
    pub amendment_text: String, // .05 AMENDMENT TEXT
}
