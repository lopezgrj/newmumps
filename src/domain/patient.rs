impl Patient {
    /// Create a new Patient with required name field
    pub fn new(name: String) -> Self {
        Self {
            name,
            sex: None,
            date_of_birth: None,
            ssn: None,
            marital_status: None,
            religion: None,
            address_line1: None,
            address_line2: None,
            address_line3: None,
            city: None,
            state: None,
            zip: None,
            county: None,
            phone_home: None,
            phone_work: None,
            next_of_kin: None,
            next_of_kin_phone: None,
            emergency_contact: None,
            emergency_contact_phone: None,
        }
    }
    /// Validate the Patient entity. Returns Ok(()) if valid, Err(String) if not.
    pub fn validate(&self) -> Result<(), String> {
        if self.name.trim().is_empty() {
            return Err("Patient name cannot be empty.".to_string());
        }
        if let Some(ref ssn) = self.ssn {
            if !ssn.chars().all(|c| c.is_digit(10) || c == '-') || ssn.len() < 9 {
                return Err("SSN must be at least 9 digits and contain only numbers or dashes.".to_string());
            }
        }
        if let Some(ref dob) = self.date_of_birth {
            // Simple ISO 8601 date check (YYYY-MM-DD)
            if !dob.chars().all(|c| c.is_digit(10) || c == '-') || dob.len() != 10 {
                return Err("Date of birth must be in YYYY-MM-DD format.".to_string());
            }
        }
        Ok(())
    }
}
