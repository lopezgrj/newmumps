// Domain entity for Clinic Recurring Appointment Pattern (VistA/MUMPS File #44.16)
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClinicRecurringAppointmentPattern {
    pub id: u32, // .01 PATTERN ID
    pub clinic_id: u32, // .02 CLINIC (pointer)
    pub pattern: String, // .03 PATTERN
    pub start_date: String, // .04 START DATE
    pub end_date: Option<String>, // .05 END DATE
}
