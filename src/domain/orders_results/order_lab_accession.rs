// Domain entity for Order Lab Accession (custom)
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderLabAccession {
    pub id: u32, // .01 ACCESSION ID
    pub order_id: u32, // .02 ORDER (pointer)
    pub accession_number: String, // .03 ACCESSION NUMBER
    pub collection_date: String, // .04 COLLECTION DATE
    pub status: String, // .05 STATUS
}
