// Domain entity for Ward, modeled after VistA/MUMPS File #42
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ward {
    pub name: String,                // .01 NAME
    pub location: Option<String>,    // 1 LOCATION
    pub bed_count: Option<u32>,      // 2 BED COUNT
    pub specialty: Option<String>,   // 3 SPECIALTY
    pub active: Option<bool>,        // 4 ACTIVE/INACTIVE
}
