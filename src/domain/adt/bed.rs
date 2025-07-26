// Domain entity for Bed (VistA/MUMPS File #405.4, bed file)
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Bed {
    pub id: u32, // .01 BED ID
    pub ward_id: u32, // .02 WARD (pointer)
    pub bed_number: String, // .03 BED NUMBER
    pub status: Option<String>, // .04 STATUS
}
