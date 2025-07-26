// Domain entity for Result Routing (custom)
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResultRouting {
    pub id: u32, // .01 ROUTING ID
    pub result_id: u32, // .02 RESULT (pointer)
    pub routed_to: u32, // .03 ROUTED TO (pointer)
    pub routing_date: String, // .04 ROUTING DATE
}
