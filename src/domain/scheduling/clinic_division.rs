// Domain entity for Clinic Division (VistA/MUMPS File #40.8)
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClinicDivision {
    pub id: u32, // .01 DIVISION ID
    pub name: String, // .01 NAME
    pub description: Option<String>, // .02 DESCRIPTION
}
