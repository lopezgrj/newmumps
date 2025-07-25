// Domain entity for Hospital Location, modeled after VistA/MUMPS File #44
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HospitalLocation {
    pub name: String,                // .01 NAME
    pub abbreviation: Option<String>,// 1 ABBREVIATION
    pub type_: Option<String>,       // 2 TYPE (Clinic, Ward, etc.)
    pub division: Option<String>,    // 3 DIVISION
    pub inactive_date: Option<String>, // 2502 INACTIVE DATE
    pub physical_location: Option<String>, // 10 PHYSICAL LOCATION
    pub telephone: Option<String>,   // 99 TELEPHONE
}
