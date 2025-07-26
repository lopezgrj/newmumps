// Domain entity for Discharge Disposition (VistA/MUMPS File #405.3)
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DischargeDisposition {
    pub id: u32, // .01 DISPOSITION ID
    pub disposition: String, // .01 DISPOSITION
    pub description: Option<String>, // .02 DESCRIPTION
}
