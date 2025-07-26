// Domain entity for Ward (VistA/MUMPS File #42)
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ward {
    pub id: u32, // .01 WARD ID
    pub name: String, // .01 NAME
    pub division_id: Option<u32>, // .02 DIVISION (pointer)
    pub status: Option<String>, // .03 STATUS
}
