// Domain entity for Patient Infection Control Event (custom)
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatientInfectionControlEvent {
    pub id: u32, // .01 EVENT ID
    pub patient_id: u32, // .02 PATIENT (pointer)
    pub event_date: String, // .03 EVENT DATE
    pub event_type: String, // .04 EVENT TYPE
    pub notes: Option<String>, // .05 NOTES
}
