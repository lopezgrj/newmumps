// Domain entity for Clinic (VistA/MUMPS File #44)
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Clinic {
    pub id: u32, // .01 CLINIC ID
    pub name: String, // .01 NAME
    pub location: Option<String>, // .02 LOCATION
    pub stop_code: Option<String>, // .03 STOP CODE
    pub active: bool, // .04 ACTIVE
}
