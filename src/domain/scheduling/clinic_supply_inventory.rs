// Domain entity for Clinic Supply Inventory (custom)
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClinicSupplyInventory {
    pub id: u32, // .01 INVENTORY ID
    pub clinic_id: u32, // .02 CLINIC (pointer)
    pub item: String, // .03 ITEM
    pub quantity: u32, // .04 QUANTITY
    pub last_updated: String, // .05 LAST UPDATED
}
