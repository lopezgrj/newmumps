// Domain entity for Clinic Resource Utilization (VistA/MUMPS File #44.18)
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClinicResourceUtilization {
    pub id: u32, // .01 UTILIZATION ID
    pub clinic_id: u32, // .02 CLINIC (pointer)
    pub resource: String, // .03 RESOURCE
    pub utilization_date: String, // .04 UTILIZATION DATE
    pub notes: Option<String>, // .05 NOTES
}
