// Domain entity for Patient Emergency Response Indicator subfile (VistA/MUMPS File #2, EMERGENCY RESPONSE INDICATOR multiple)
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatientEmergencyResponseIndicator {
    pub indicator: String, // .01 INDICATOR
    pub date_recorded: Option<String>, // .02 DATE RECORDED
}
