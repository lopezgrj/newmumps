// Domain entity for Result Comment (VistA/MUMPS File #100, result comment subfile)
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResultComment {
    pub id: u32, // .01 COMMENT ID
    pub result_id: u32, // .02 RESULT (pointer)
    pub comment_text: String, // .03 COMMENT TEXT
    pub entered_by: Option<u32>, // .04 ENTERED BY (pointer)
    pub entry_date: Option<String>, // .05 ENTRY DATE
}
