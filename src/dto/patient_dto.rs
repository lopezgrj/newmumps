use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct PatientInput {
    pub name: String,
    pub sex: Option<String>,
    pub date_of_birth: Option<String>,
    pub ssn: Option<String>,
    pub marital_status: Option<String>,
    pub religion: Option<String>,
    pub address_line1: Option<String>,
    pub address_line2: Option<String>,
    pub address_line3: Option<String>,
    pub city: Option<String>,
    pub state: Option<String>,
    pub zip: Option<String>,
    pub county: Option<String>,
    pub phone_home: Option<String>,
    pub phone_work: Option<String>,
    pub next_of_kin: Option<String>,
    pub next_of_kin_phone: Option<String>,
    pub emergency_contact: Option<String>,
    pub emergency_contact_phone: Option<String>,
}

#[derive(Serialize)]
pub struct PatientOut {
    pub id: i64,
    pub name: String,
    pub sex: Option<String>,
    pub date_of_birth: Option<String>,
    pub ssn: Option<String>,
    pub marital_status: Option<String>,
    pub religion: Option<String>,
    pub address_line1: Option<String>,
    pub address_line2: Option<String>,
    pub address_line3: Option<String>,
    pub city: Option<String>,
    pub state: Option<String>,
    pub zip: Option<String>,
    pub county: Option<String>,
    pub phone_home: Option<String>,
    pub phone_work: Option<String>,
    pub next_of_kin: Option<String>,
    pub next_of_kin_phone: Option<String>,
    pub emergency_contact: Option<String>,
    pub emergency_contact_phone: Option<String>,
}
