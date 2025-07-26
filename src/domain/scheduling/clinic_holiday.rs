// Domain entity for Clinic Holiday (VistA/MUMPS File #44.1)
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClinicHoliday {
    pub id: u32, // .01 HOLIDAY ID
    pub clinic_id: u32, // .02 CLINIC (pointer)
    pub holiday_date: String, // .03 HOLIDAY DATE
    pub description: Option<String>, // .04 DESCRIPTION
}
