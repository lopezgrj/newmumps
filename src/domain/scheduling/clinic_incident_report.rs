// Domain entity for Clinic Incident Report (custom)
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClinicIncidentReport {
    pub id: u32, // .01 REPORT ID
    pub clinic_id: u32, // .02 CLINIC (pointer)
    pub report_date: String, // .03 REPORT DATE
    pub description: String, // .04 DESCRIPTION
    pub reported_by: u32, // .05 REPORTED BY (pointer)
}
