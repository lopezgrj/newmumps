// Domain entity for Order Imaging Request (custom)
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderImagingRequest {
    pub id: u32, // .01 REQUEST ID
    pub order_id: u32, // .02 ORDER (pointer)
    pub imaging_type: String, // .03 IMAGING TYPE
    pub request_date: String, // .04 REQUEST DATE
    pub status: String, // .05 STATUS
}
