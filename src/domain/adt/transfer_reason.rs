// Domain entity for Transfer Reason (VistA/MUMPS File #405.2)
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransferReason {
    pub id: u32, // .01 REASON ID
    pub reason: String, // .01 REASON
    pub description: Option<String>, // .02 DESCRIPTION
}
