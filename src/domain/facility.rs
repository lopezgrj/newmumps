// Domain entity for Facility (File #5 in VistA)
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Facility {
    pub name: String,                // .01 NAME
    pub type_code: Option<String>,   // .02 TYPE
    pub address_line1: Option<String>, // .111 STREET ADDRESS [LINE 1]
    pub city: Option<String>,        // .114 CITY
    pub state: Option<String>,       // .115 STATE
    pub zip: Option<String>,         // .116 ZIP+4
    pub phone: Option<String>,       // .131 PHONE NUMBER
}

impl Facility {
    /// Create a new Facility with required name field
    pub fn new(name: String) -> Self {
        Self {
            name,
            type_code: None,
            address_line1: None,
            city: None,
            state: None,
            zip: None,
            phone: None,
        }
    }

    /// Validate the Facility entity. Returns Ok(()) if valid, Err(String) if not.
    pub fn validate(&self) -> Result<(), String> {
        if self.name.trim().is_empty() {
            return Err("Facility name cannot be empty.".to_string());
        }
        if let Some(ref zip) = self.zip {
            if zip.len() < 5 {
                return Err("ZIP code must be at least 5 characters.".to_string());
            }
        }
        Ok(())
    }
}
