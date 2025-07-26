// Domain entity for Order Dialog (VistA/MUMPS File #101.41)
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderDialog {
    pub id: u32, // .01 DIALOG ID
    pub name: String, // .01 NAME
    pub description: Option<String>, // .02 DESCRIPTION
    pub type_code: Option<String>, // .03 TYPE
}
