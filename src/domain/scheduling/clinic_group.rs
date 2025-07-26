// Domain entity for Clinic Group (VistA/MUMPS File #409.61)
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClinicGroup {
    pub id: u32, // .01 GROUP ID
    pub name: String, // .01 NAME
    pub description: Option<String>, // .02 DESCRIPTION
}
