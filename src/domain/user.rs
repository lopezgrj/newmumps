// Domain entity for User (File #3 in VistA)
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub name: String,                // .01 NAME
    pub title: Option<String>,       // .02 TITLE
    pub service_section: Option<String>, // .03 SERVICE/SECTION
    pub ssn: Option<String>,         // .09 SOCIAL SECURITY NUMBER
    pub dob: Option<String>,         // .03 DATE OF BIRTH
    pub sex: Option<String>,         // .02 SEX
    pub phone: Option<String>,       // .132 PHONE NUMBER [WORK]
    pub email: Option<String>,       // .151 EMAIL ADDRESS
}

impl User {
    /// Create a new User with required name field
    pub fn new(name: String) -> Self {
        Self {
            name,
            title: None,
            service_section: None,
            ssn: None,
            dob: None,
            sex: None,
            phone: None,
            email: None,
        }
    }

    /// Validate the User entity. Returns Ok(()) if valid, Err(String) if not.
    pub fn validate(&self) -> Result<(), String> {
        if self.name.trim().is_empty() {
            return Err("User name cannot be empty.".to_string());
        }
        if let Some(ref ssn) = self.ssn {
            if !ssn.chars().all(|c| c.is_digit(10) || c == '-') || ssn.len() < 9 {
                return Err("SSN must be at least 9 digits and contain only numbers or dashes.".to_string());
            }
        }
        if let Some(ref dob) = self.dob {
            if !dob.chars().all(|c| c.is_digit(10) || c == '-') || dob.len() != 10 {
                return Err("Date of birth must be in YYYY-MM-DD format.".to_string());
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
