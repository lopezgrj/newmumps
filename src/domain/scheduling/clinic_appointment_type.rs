// Domain entity for Clinic Appointment Type (VistA/MUMPS File #409.1)
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClinicAppointmentType {
    pub id: u32, // .01 APPOINTMENT TYPE ID
    pub name: String, // .01 NAME
    pub description: Option<String>, // .02 DESCRIPTION
}
