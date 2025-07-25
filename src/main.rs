mod domain {
    pub mod patient;
    pub mod user;
    pub mod institution;
    pub mod facility;
    pub mod provider;
}

use crate::domain::patient::Patient;

fn main() {
    let patient = Patient {
        name: "John Doe".to_string(),
        sex: Some("M".to_string()),
        date_of_birth: Some("1980-01-01".to_string()),
        ssn: Some("123-45-6789".to_string()),
        marital_status: Some("Single".to_string()),
        religion: Some("None".to_string()),
        address_line1: Some("123 Main St".to_string()),
        address_line2: None,
        address_line3: None,
        city: Some("Anytown".to_string()),
        state: Some("NY".to_string()),
        zip: Some("12345".to_string()),
        county: Some("AnyCounty".to_string()),
        phone_home: Some("555-1234".to_string()),
        phone_work: None,
        next_of_kin: Some("Jane Doe".to_string()),
        next_of_kin_phone: Some("555-5678".to_string()),
        emergency_contact: Some("Jim Smith".to_string()),
        emergency_contact_phone: Some("555-8765".to_string()),
    };
    println!("Patient: {:#?}", patient);
}
