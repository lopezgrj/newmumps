// Domain entity for Patient Preferred Facility subfile (VistA/MUMPS File #2, PREFERRED FACILITY multiple)
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatientPreferredFacility {
    pub facility_id: u32, // .01 FACILITY (pointer)
    pub start_date: Option<String>, // .02 START DATE
    pub end_date: Option<String>, // .03 END DATE
}
