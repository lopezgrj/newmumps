// Domain entity for Protocol, modeled after VistA/MUMPS File #101
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Protocol {
    pub name: String,                // .01 NAME
    pub type_: Option<String>,       // 2 TYPE
    pub description: Option<String>, // 3 DESCRIPTION
    pub active: Option<bool>,        // 4 ACTIVE/INACTIVE
}
