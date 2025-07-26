// Domain entity for Result (VistA/MUMPS File #100, result subfile)
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Result {
    pub id: u32, // .01 RESULT ID
    pub order_id: u32, // .02 ORDER (pointer)
    pub result_date: String, // .03 RESULT DATE
    pub result_text: String, // .04 RESULT TEXT
    pub status: String, // .05 STATUS
}
