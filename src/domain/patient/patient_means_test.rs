// Domain entity for Patient Means Test subfile (VistA/MUMPS File #2, MEANS TEST multiple)
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatientMeansTest {
    pub test_date: String, // .01 TEST DATE
    pub status: String, // .02 STATUS
    pub entered_by: Option<u32>, // .03 ENTERED BY (pointer)
}
