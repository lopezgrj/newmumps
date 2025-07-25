// Domain entity for Patient Purple Heart Status subfile (VistA/MUMPS File #2, PURPLE HEART STATUS multiple)
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatientPurpleHeartStatus {
    pub award_date: Option<String>, // .01 AWARD DATE
    pub remarks: Option<String>, // .02 REMARKS
}
