// Domain entity for Orderable Item Synonym (VistA/MUMPS File #101.44)
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderableItemSynonym {
    pub id: u32, // .01 SYNONYM ID
    pub item_id: u32, // .02 ITEM (pointer)
    pub synonym: String, // .03 SYNONYM
}
