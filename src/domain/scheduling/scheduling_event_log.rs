// Domain entity for Scheduling Event Log, modeled after VistA/MUMPS File #409.41
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SchedulingEventLog {
    pub event_id: u32,               // .01 EVENT (pointer)
    pub log_date: String,            // .02 LOG DATE
    pub user_id: Option<u32>,        // .03 USER (pointer)
    pub description: Option<String>, // 1 DESCRIPTION
}
