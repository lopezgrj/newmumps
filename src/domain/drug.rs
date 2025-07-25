// Domain entity for Drug, modeled after VistA/MUMPS File #50
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Drug {
    pub name: String,                // .01 NAME
    pub generic_name: Option<String>,// 1 GENERIC NAME
    pub va_product_name: Option<String>, // 2 VA PRODUCT NAME
    pub strength: Option<String>,    // 3 STRENGTH
    pub unit: Option<String>,        // 4 UNIT
    pub inactive: Option<bool>,      // 100 INACTIVE
}
