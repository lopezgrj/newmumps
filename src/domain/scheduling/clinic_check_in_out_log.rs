// Domain entity for Clinic Check-in/Check-out Log (VistA/MUMPS File #44.09)
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClinicCheckInOutLog {
    pub id: u32, // .01 LOG ID
    pub clinic_id: u32, // .02 CLINIC (pointer)
    pub patient_id: u32, // .03 PATIENT (pointer)
    pub check_in_time: String, // .04 CHECK-IN TIME
    pub check_out_time: Option<String>, // .05 CHECK-OUT TIME
}
