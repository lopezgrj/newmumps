// Domain entity for Purpose of Visit (POV), modeled after VistA/MUMPS File #9000010.07
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PurposeOfVisit {
    pub visit_id: u32,               // .01 VISIT (pointer)
    pub diagnosis_code: String,      // .02 DIAGNOSIS CODE
    pub narrative: Option<String>,   // .04 NARRATIVE
    pub provider_id: Option<u32>,    // .05 PROVIDER (pointer)
    pub primary: Option<bool>,       // .12 PRIMARY
}
