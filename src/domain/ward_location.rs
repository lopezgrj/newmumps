// Domain entity for Ward Location, modeled after VistA/MUMPS File #16
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WardLocation {
    /// .01 NAME - Ward Location Name
    pub name: String,
    /// 1 ABBREVIATION - Short name or code for the ward
    pub abbreviation: Option<String>,
    /// 2502 INACTIVE DATE - Date ward location became inactive
    pub inactive_date: Option<String>,
    /// 10 PHYSICAL LOCATION - Physical location/description
    pub physical_location: Option<String>,
    /// 99 TELEPHONE - Contact phone number for the ward
    pub telephone: Option<String>,
}
