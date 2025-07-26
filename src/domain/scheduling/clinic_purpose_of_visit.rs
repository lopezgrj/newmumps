// Domain entity for Clinic Purpose of Visit (VistA/MUMPS File #409.61)
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClinicPurposeOfVisit {
    pub id: u32, // .01 PURPOSE OF VISIT ID
    pub name: String, // .01 NAME
    pub description: Option<String>, // .02 DESCRIPTION
}
