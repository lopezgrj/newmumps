// Domain entity for Institution (File #4 in VistA)
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Institution {
    /// .01 NAME - Institution Name
    pub name: String,
    /// .99 STATION NUMBER - VA Station Number
    pub station_number: Option<String>,
    /// .111 STREET ADDRESS [LINE 1]
    pub address_line1: Option<String>,
    /// .114 CITY
    pub city: Option<String>,
    /// .115 STATE
    pub state: Option<String>,
    /// .116 ZIP+4
    pub zip: Option<String>,
    /// .131 PHONE NUMBER
    pub phone: Option<String>,
}

impl Institution {
    /// Create a new Institution with required name field
    pub fn new(name: String) -> Self {
        Self {
            name,
            station_number: None,
            address_line1: None,
            city: None,
            state: None,
            zip: None,
            phone: None,
        }
    }

    /// Validate the Institution entity. Returns Ok(()) if valid, Err(String) if not.
    pub fn validate(&self) -> Result<(), String> {
        if self.name.trim().is_empty() {
            return Err("Institution name cannot be empty.".to_string());
        }
        if let Some(ref zip) = self.zip {
            if zip.len() < 5 {
                return Err("ZIP code must be at least 5 characters.".to_string());
            }
        }
        Ok(())
    }
}
