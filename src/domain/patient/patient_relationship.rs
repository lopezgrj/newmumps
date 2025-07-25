// Domain entity for Patient Relationship, modeled after VistA/MUMPS File #8.1
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatientRelationship {
    pub patient_id: u32,             // .01 PATIENT (pointer)
    pub related_person_id: u32,      // .02 RELATED PERSON (pointer)
    pub relationship_type: String,   // .03 RELATIONSHIP TYPE
    pub start_date: Option<String>,  // .04 START DATE
    pub end_date: Option<String>,    // .05 END DATE
}
