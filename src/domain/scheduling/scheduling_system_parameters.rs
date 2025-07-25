// Domain entity for Scheduling System Parameters, modeled after VistA/MUMPS File #409.44
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SchedulingSystemParameters {
    pub name: String,                // .01 NAME
    pub value: Option<String>,       // 1 VALUE
    pub description: Option<String>, // 2 DESCRIPTION
}
