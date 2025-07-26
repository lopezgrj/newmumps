// Domain entity for Patient Scheduling Preference (custom subfile)
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatientSchedulingPreference {
    pub patient_id: u32, // .01 PATIENT (pointer)
    pub preferred_days: Option<String>, // .02 PREFERRED DAYS
    pub preferred_times: Option<String>, // .03 PREFERRED TIMES
    pub notes: Option<String>, // .04 NOTES
}
