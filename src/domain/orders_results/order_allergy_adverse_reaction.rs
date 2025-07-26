// Domain entity for Order Allergy/Adverse Reaction (custom)
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderAllergyAdverseReaction {
    pub id: u32, // .01 REACTION ID
    pub order_id: u32, // .02 ORDER (pointer)
    pub reaction: String, // .03 REACTION
    pub reaction_date: String, // .04 REACTION DATE
    pub severity: Option<String>, // .05 SEVERITY
}
