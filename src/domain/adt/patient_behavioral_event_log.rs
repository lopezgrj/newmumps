// Domain entity for Patient Behavioral Event Log (custom)
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatientBehavioralEventLog {
    pub id: u32, // .01 LOG ID
    pub patient_id: u32, // .02 PATIENT (pointer)
    pub event_date: String, // .03 EVENT DATE
    pub description: String, // .04 DESCRIPTION
    pub reported_by: Option<u32>, // .05 REPORTED BY (pointer)
}
