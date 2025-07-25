// Domain entity for Appointment Request Reason, modeled after VistA/MUMPS File #409.85
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppointmentRequestReason {
    pub name: String,                // .01 NAME
    pub description: Option<String>, // 1 DESCRIPTION
    pub active: Option<bool>,        // 2 ACTIVE/INACTIVE
}
