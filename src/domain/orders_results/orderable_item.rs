// Domain entity for Orderable Item (VistA/MUMPS File #101.43)
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderableItem {
    pub id: u32, // .01 ITEM ID
    pub name: String, // .01 NAME
    pub synonym: Option<String>, // .02 SYNONYM
    pub type_code: Option<String>, // .03 TYPE
    pub status: Option<String>, // .04 STATUS
}
