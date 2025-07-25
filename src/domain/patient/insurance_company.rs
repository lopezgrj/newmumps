// Domain entity for Insurance Company, modeled after VistA/MUMPS File #40
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InsuranceCompany {
    pub name: String,                // .01 NAME
    pub address: Option<String>,     // 1 ADDRESS
    pub phone: Option<String>,       // 2 PHONE
    pub active: Option<bool>,        // 3 ACTIVE/INACTIVE
}
