// Domain entity for Clinic Outreach Event (custom)
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClinicOutreachEvent {
    pub id: u32, // .01 EVENT ID
    pub clinic_id: u32, // .02 CLINIC (pointer)
    pub event_date: String, // .03 EVENT DATE
    pub description: String, // .04 DESCRIPTION
}
