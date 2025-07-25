// Domain entity for Insurance Policy, modeled after VistA/MUMPS File #39.1
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InsurancePolicy {
    pub patient_id: u32,             // .01 PATIENT (pointer)
    pub company_id: u32,             // .02 INSURANCE COMPANY (pointer)
    pub policy_number: String,       // .03 POLICY NUMBER
    pub effective_date: Option<String>, // .04 EFFECTIVE DATE
    pub expiration_date: Option<String>, // .05 EXPIRATION DATE
}
