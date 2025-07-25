// Domain entity for Patient Exposure Record subfile (VistA/MUMPS File #2, EXPOSURE RECORD multiple)
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatientExposureRecord {
    pub exposure_type: String, // .01 EXPOSURE TYPE (e.g., Agent Orange, Radiation)
    pub exposure_date: Option<String>, // .02 EXPOSURE DATE
    pub remarks: Option<String>, // .03 REMARKS
}
