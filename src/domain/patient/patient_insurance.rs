// Domain entity for Patient Insurance subfile (VistA/MUMPS File #2, INSURANCE multiple)
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatientInsurance {
    pub insurance_company_id: u32, // .01 INSURANCE COMPANY (pointer)
    pub policy_number: Option<String>, // .02 POLICY NUMBER
    pub group_number: Option<String>, // .03 GROUP NUMBER
    pub effective_date: Option<String>, // .04 EFFECTIVE DATE
    pub expiration_date: Option<String>, // .05 EXPIRATION DATE
    pub relationship: Option<String>, // .06 RELATIONSHIP TO INSURED
}
