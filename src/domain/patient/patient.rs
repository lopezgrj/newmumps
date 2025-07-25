// Domain entity for Patient, modeled after VistA/MUMPS File #2
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Patient {
    pub name: String,                // .01 NAME
    pub sex: Option<String>,         // .02 SEX
    pub date_of_birth: Option<String>, // .03 DATE OF BIRTH (ISO 8601 recommended)
    pub ssn: Option<String>,         // .09 SOCIAL SECURITY NUMBER
    pub marital_status: Option<String>, // .05 MARITAL STATUS
    pub religion: Option<String>,    // .08 RELIGION
    pub address_line1: Option<String>, // .111 STREET ADDRESS [LINE 1]
    pub address_line2: Option<String>, // .112 STREET ADDRESS [LINE 2]
    pub address_line3: Option<String>, // .113 STREET ADDRESS [LINE 3]
    pub city: Option<String>,        // .114 CITY
    pub state: Option<String>,       // .115 STATE
    pub zip: Option<String>,         // .116 ZIP+4
    pub county: Option<String>,      // .117 COUNTY
    pub phone_home: Option<String>,  // .131 PHONE NUMBER [HOME]
    pub phone_work: Option<String>,  // .132 PHONE NUMBER [WORK]
    pub next_of_kin: Option<String>, // .211 NEXT OF KIN
    pub next_of_kin_phone: Option<String>, // .219 NEXT OF KIN PHONE NUMBER
    pub emergency_contact: Option<String>, // .2402 EMERGENCY CONTACT
    pub emergency_contact_phone: Option<String>, // .2403 EMERGENCY CONTACT PHONE
}
