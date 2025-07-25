// Domain entity for Patient Race, modeled after VistA/MUMPS File #27.17
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatientRace {
    pub patient_id: u32,             // .01 PATIENT (pointer)
    pub race: String,                // .02 RACE
    pub entered_date: Option<String>,// .03 ENTERED DATE
}
