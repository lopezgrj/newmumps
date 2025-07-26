// Domain entity for Result Flag (custom)
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResultFlag {
    pub id: u32, // .01 FLAG ID
    pub result_id: u32, // .02 RESULT (pointer)
    pub flag_type: String, // .03 FLAG TYPE (abnormal, critical, etc.)
    pub flagged_by: Option<u32>, // .04 FLAGGED BY (pointer)
    pub flag_date: Option<String>, // .05 FLAG DATE
}
