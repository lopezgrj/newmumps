// Domain entity for Scheduling Profile, modeled after VistA/MUMPS File #409.32
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SchedulingProfile {
    pub name: String,                // .01 NAME
    pub description: Option<String>, // 1 DESCRIPTION
    pub active: Option<bool>,        // 2 ACTIVE/INACTIVE
}
