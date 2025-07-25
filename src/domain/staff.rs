// Domain entity for Staff, modeled after VistA/MUMPS File #8
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Staff {
    pub name: String,                // .01 NAME
    pub title: Option<String>,       // 2 TITLE
    pub ssn: Option<String>,         // 9 SSN
    pub service: Option<String>,     // 29 SERVICE/SECTION
    pub active: Option<bool>,        // 53.4 ACTIVE/INACTIVE
}
