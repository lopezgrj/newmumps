// Domain entity for Discharge (VistA/MUMPS File #405, discharge movement)
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Discharge {
    pub id: u32, // .01 DISCHARGE ID
    pub patient_id: u32, // .02 PATIENT (pointer)
    pub discharge_date: String, // .03 DISCHARGE DATE
    pub discharge_location: Option<String>, // .04 DISCHARGE LOCATION
    pub discharge_provider: Option<u32>, // .05 DISCHARGE PROVIDER (pointer)
    pub discharge_type: Option<String>, // .06 DISCHARGE TYPE
}
