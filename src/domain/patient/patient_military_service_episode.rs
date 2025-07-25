// Domain entity for Patient Military Service Episode subfile (VistA/MUMPS File #2, MILITARY SERVICE EPISODE multiple)
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatientMilitaryServiceEpisode {
    pub branch_of_service: String, // .01 BRANCH OF SERVICE
    pub service_number: Option<String>, // .02 SERVICE NUMBER
    pub entry_date: Option<String>, // .03 ENTRY DATE
    pub separation_date: Option<String>, // .04 SEPARATION DATE
    pub discharge_type: Option<String>, // .05 DISCHARGE TYPE
}
