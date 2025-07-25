// Domain entity for Eligibility Code, modeled after VistA/MUMPS File #38
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EligibilityCode {
    pub name: String,                // .01 NAME
    pub description: Option<String>, // 1 DESCRIPTION
    pub active: Option<bool>,        // 2 ACTIVE/INACTIVE
}
