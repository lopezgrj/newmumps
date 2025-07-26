// Domain entity for Order Pharmacy Dispense (custom)
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderPharmacyDispense {
    pub id: u32, // .01 DISPENSE ID
    pub order_id: u32, // .02 ORDER (pointer)
    pub drug_name: String, // .03 DRUG NAME
    pub dispense_date: String, // .04 DISPENSE DATE
    pub quantity: u32, // .05 QUANTITY
    pub status: String, // .06 STATUS
}
