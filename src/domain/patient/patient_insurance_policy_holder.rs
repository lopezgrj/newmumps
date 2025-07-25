// Domain entity for Patient Insurance Policy Holder subfile (VistA/MUMPS File #2, INSURANCE POLICY HOLDER multiple)
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatientInsurancePolicyHolder {
    pub holder_name: String, // .01 POLICY HOLDER NAME
    pub relationship: Option<String>, // .02 RELATIONSHIP TO PATIENT
    pub policy_number: Option<String>, // .03 POLICY NUMBER
}
