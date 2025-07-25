// Domain entity for Treating Specialty, modeled after VistA/MUMPS File #405.1
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TreatingSpecialty {
    pub name: String,                // .01 NAME
    pub abbreviation: Option<String>,// 1 ABBREVIATION
    pub service: Option<String>,     // 2 SERVICE
    pub active: Option<bool>,        // 3 ACTIVE/INACTIVE
}
