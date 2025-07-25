// Domain entity for Clinic Stop, modeled after VistA/MUMPS File #409.3
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClinicStop {
    pub name: String,                // .01 NAME
    pub code: Option<String>,        // 1 CODE
    pub type_: Option<String>,       // 2 TYPE
    pub active: Option<bool>,        // 3 ACTIVE/INACTIVE
}
