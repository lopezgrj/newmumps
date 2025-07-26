// Domain entity for Patient Organ Donor Status subfile (VistA/MUMPS File #2, ORGAN DONOR STATUS multiple)
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatientOrganDonorStatus {
    pub status: String, // .01 STATUS
    pub date_recorded: Option<String>, // .02 DATE RECORDED
}
