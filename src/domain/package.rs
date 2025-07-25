// Domain entity for Package, modeled after VistA/MUMPS File #9.4
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Package {
    pub name: String,                // .01 NAME
    pub prefix: Option<String>,      // 1 PREFIX
    pub version: Option<String>,     // 2 VERSION
    pub description: Option<String>, // 3 DESCRIPTION
    pub date_installed: Option<String>, // 4 DATE INSTALLED
}
