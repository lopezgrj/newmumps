// Domain entity for Bed Status History (custom)
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BedStatusHistory {
    pub id: u32, // .01 HISTORY ID
    pub bed_id: u32, // .02 BED (pointer)
    pub status: String, // .03 STATUS
    pub start_date: String, // .04 START DATE
    pub end_date: Option<String>, // .05 END DATE
}
