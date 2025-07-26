// Domain entity for Patient Community Care Episode subfile (VistA/MUMPS File #2, COMMUNITY CARE EPISODE multiple)
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatientCommunityCareEpisode {
    pub episode_type: String, // .01 EPISODE TYPE
    pub start_date: Option<String>, // .02 START DATE
    pub end_date: Option<String>, // .03 END DATE
    pub provider: Option<String>, // .04 PROVIDER
}
