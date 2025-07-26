// Domain entity for ADT Event Log (custom)
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdtEventLog {
    pub id: u32, // .01 EVENT ID
    pub patient_id: u32, // .02 PATIENT (pointer)
    pub event_type: String, // .03 EVENT TYPE
    pub event_date: String, // .04 EVENT DATE
    pub user_id: Option<u32>, // .05 USER (pointer)
    pub notes: Option<String>, // .06 NOTES
}
