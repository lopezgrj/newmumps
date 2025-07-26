// Domain entity for Ward Location History (custom)
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WardLocationHistory {
    pub id: u32, // .01 HISTORY ID
    pub patient_id: u32, // .02 PATIENT (pointer)
    pub ward_id: u32, // .03 WARD (pointer)
    pub start_date: String, // .04 START DATE
    pub end_date: Option<String>, // .05 END DATE
}
