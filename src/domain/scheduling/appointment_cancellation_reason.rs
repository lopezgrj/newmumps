// Domain entity for Appointment Cancellation Reason, modeled after VistA/MUMPS File #409.84
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppointmentCancellationReason {
    pub name: String,                // .01 NAME
    pub description: Option<String>, // 1 DESCRIPTION
    pub active: Option<bool>,        // 2 ACTIVE/INACTIVE
}
