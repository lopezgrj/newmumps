// Domain entity for Result Attachment (custom)
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResultAttachment {
    pub id: u32, // .01 ATTACHMENT ID
    pub result_id: u32, // .02 RESULT (pointer)
    pub file_name: String, // .03 FILE NAME
    pub file_type: String, // .04 FILE TYPE
    pub uploaded_by: u32, // .05 UPLOADED BY (pointer)
    pub upload_date: String, // .06 UPLOAD DATE
}
