// Domain entity for Provider (File #6 in VistA)
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Provider {
    pub name: String,                // .01 NAME
    pub npi: Option<String>,         // .104 NATIONAL PROVIDER IDENTIFIER
    pub specialty: Option<String>,   // .08 SPECIALTY
    pub phone: Option<String>,       // .131 PHONE NUMBER
    pub email: Option<String>,       // .151 EMAIL ADDRESS
}

impl Provider {
    /// Create a new Provider with required name field
    pub fn new(name: String) -> Self {
        Self {
            name,
            npi: None,
            specialty: None,
            phone: None,
            email: None,
        }
    }

    /// Validate the Provider entity. Returns Ok(()) if valid, Err(String) if not.
    pub fn validate(&self) -> Result<(), String> {
        if self.name.trim().is_empty() {
            return Err("Provider name cannot be empty.".to_string());
        }
        if let Some(ref npi) = self.npi {
            if !npi.chars().all(|c| c.is_digit(10)) || npi.len() != 10 {
                return Err("NPI must be exactly 10 digits.".to_string());
            }
        }
        if let Some(ref email) = self.email {
            if !email.contains('@') {
                return Err("Email must contain '@'.".to_string());
            }
        }
        Ok(())
    }
}
