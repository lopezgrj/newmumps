// Domain entity for Clinic Emergency Drill Log (custom)
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClinicEmergencyDrillLog {
    pub id: u32, // .01 DRILL ID
    pub clinic_id: u32, // .02 CLINIC (pointer)
    pub drill_date: String, // .03 DRILL DATE
    pub type_of_drill: String, // .04 TYPE OF DRILL
    pub notes: Option<String>, // .05 NOTES
}
