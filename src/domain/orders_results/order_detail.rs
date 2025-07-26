// Domain entity for Order Detail (VistA/MUMPS File #100, subfile)
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderDetail {
    pub id: u32, // .01 DETAIL ID
    pub order_id: u32, // .02 ORDER (pointer)
    pub detail_text: String, // .03 DETAIL TEXT
}
