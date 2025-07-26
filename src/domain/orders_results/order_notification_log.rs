// Domain entity for Order Notification Log (custom)
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderNotificationLog {
    pub id: u32, // .01 LOG ID
    pub order_id: u32, // .02 ORDER (pointer)
    pub notification_date: String, // .03 NOTIFICATION DATE
    pub notification_type: String, // .04 NOTIFICATION TYPE
    pub user_id: Option<u32>, // .05 USER (pointer)
    pub notes: Option<String>, // .06 NOTES
}
