// Domain entity for Scheduling Reason, modeled after VistA/MUMPS File #409.2
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SchedulingReason {
    pub name: String,                // .01 NAME
    pub type_: Option<String>,       // 1 TYPE
    pub active: Option<bool>,        // 2 ACTIVE/INACTIVE
}
