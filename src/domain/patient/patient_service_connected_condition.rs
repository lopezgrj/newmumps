// Domain entity for Patient Service Connected Condition subfile (VistA/MUMPS File #2, SERVICE CONNECTED CONDITION multiple)
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatientServiceConnectedCondition {
    pub condition: String, // .01 CONDITION
    pub sc_percent: Option<u8>, // .02 SERVICE CONNECTED PERCENTAGE
}
