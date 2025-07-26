// Domain entity for Result External Reference (custom)
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResultExternalReference {
    pub id: u32, // .01 REFERENCE ID
    pub result_id: u32, // .02 RESULT (pointer)
    pub external_system: String, // .03 EXTERNAL SYSTEM
    pub reference_id: String, // .04 REFERENCE ID
    pub reference_date: String, // .05 REFERENCE DATE
}
