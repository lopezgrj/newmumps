// Domain entity for Order Protocol (VistA/MUMPS File #101)
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderProtocol {
    pub id: u32, // .01 PROTOCOL ID
    pub name: String, // .01 NAME
    pub description: Option<String>, // .02 DESCRIPTION
    pub type_code: Option<String>, // .03 TYPE
}
