// Domain entity for Clinic Appointment Wait Time (VistA/MUMPS File #44.20)
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClinicAppointmentWaitTime {
    pub id: u32, // .01 WAIT TIME ID
    pub appointment_id: u32, // .02 APPOINTMENT (pointer)
    pub wait_time_minutes: u32, // .03 WAIT TIME (minutes)
    pub recorded_date: String, // .04 RECORDED DATE
}
