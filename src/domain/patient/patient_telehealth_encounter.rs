// Domain entity for Patient Telehealth Encounter subfile (VistA/MUMPS File #2, TELEHEALTH ENCOUNTER multiple)
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatientTelehealthEncounter {
    pub encounter_date: String, // .01 ENCOUNTER DATE
    pub provider: Option<String>, // .02 PROVIDER
    pub modality: Option<String>, // .03 MODALITY
    pub notes: Option<String>, // .04 NOTES
}
