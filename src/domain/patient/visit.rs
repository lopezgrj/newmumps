// Domain entity for Visit, modeled after VistA/MUMPS File #9000010
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Visit {
    pub patient_id: u32,             // .01 PATIENT (pointer)
    pub visit_date: String,          // .03 VISIT DATE
    pub location: Option<String>,    // .22 LOCATION
    pub service_category: Option<String>, // .07 SERVICE CATEGORY
    pub type_: Option<String>,       // .24 TYPE
    pub provider_id: Option<u32>,    // .08 PROVIDER (pointer)
    pub purpose: Option<String>,     // .18 PURPOSE
}
