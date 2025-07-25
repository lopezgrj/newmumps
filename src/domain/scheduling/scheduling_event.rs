// Domain entity for Scheduling Event, modeled after VistA/MUMPS File #409.42
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SchedulingEvent {
    pub event_type_id: u32,          // .01 EVENT TYPE (pointer)
    pub event_date: String,          // .02 EVENT DATE
    pub user_id: Option<u32>,        // .03 USER (pointer)
    pub description: Option<String>, // 1 DESCRIPTION
}
