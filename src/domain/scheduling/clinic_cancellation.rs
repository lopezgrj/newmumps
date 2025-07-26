// Domain entity for Clinic Cancellation (VistA/MUMPS File #409.2)
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClinicCancellation {
    pub id: u32, // .01 CANCELLATION ID
    pub clinic_id: u32, // .02 CLINIC (pointer)
    pub cancellation_date: String, // .03 CANCELLATION DATE
    pub reason: Option<String>, // .04 REASON
}
