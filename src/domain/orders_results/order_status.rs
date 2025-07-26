// Domain entity for Order Status (VistA/MUMPS File #100.01)
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderStatus {
    pub id: u32, // .01 STATUS ID
    pub name: String, // .01 NAME
    pub description: Option<String>, // .02 DESCRIPTION
}
