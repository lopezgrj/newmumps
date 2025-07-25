// Domain entity for Patient Income subfile (VistA/MUMPS File #2, INCOME multiple)
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatientIncome {
    pub year: u16, // .01 YEAR
    pub amount: f64, // .02 AMOUNT
    pub source: Option<String>, // .03 SOURCE
}
