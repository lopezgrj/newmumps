// Domain entity for Resource, modeled after VistA/MUMPS File #403.46
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Resource {
    pub name: String,                // .01 NAME
    pub type_: Option<String>,       // 2 TYPE
    pub active: Option<bool>,        // 3 ACTIVE/INACTIVE
}
