// Domain entity for Appointment Request Action Type, modeled after VistA/MUMPS File #409.89
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppointmentRequestActionType {
    pub name: String,                // .01 NAME
    pub description: Option<String>, // 1 DESCRIPTION
    pub active: Option<bool>,        // 2 ACTIVE/INACTIVE
}
