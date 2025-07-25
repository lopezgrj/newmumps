// Domain entity for New Person, modeled after VistA/MUMPS File #200
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewPerson {
    pub name: String,                // .01 NAME
    pub initials: Option<String>,    // 1 INITIALS
    pub ssn: Option<String>,         // 9 SSN
    pub title: Option<String>,       // 8 TITLE
    pub service_section: Option<String>, // 29 SERVICE/SECTION
    pub class: Option<String>,       // 9.5 CLASS
    pub termination_date: Option<String>, // 9.2 TERMINATION DATE
    pub dea_number: Option<String>,  // 53 DEA #
    pub npi: Option<String>,         // 41.99 NPI
    pub active: Option<bool>,        // 53.4 ACTIVE/INACTIVE
}
