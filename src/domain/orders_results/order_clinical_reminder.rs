// Domain entity for Order Clinical Reminder (custom)
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderClinicalReminder {
    pub id: u32, // .01 REMINDER ID
    pub order_id: u32, // .02 ORDER (pointer)
    pub reminder_text: String, // .03 REMINDER TEXT
    pub due_date: Option<String>, // .04 DUE DATE
}
