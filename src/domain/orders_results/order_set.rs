// Domain entity for Order Set (VistA/MUMPS File #100.21)
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderSet {
    pub id: u32, // .01 SET ID
    pub name: String, // .01 NAME
    pub description: Option<String>, // .02 DESCRIPTION
    pub status: Option<String>, // .03 STATUS
}
