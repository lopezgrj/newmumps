// Domain entity for Radiology Exam, modeled after VistA/MUMPS File #70
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RadiologyExam {
    pub patient_id: u32,             // .01 PATIENT (pointer)
    pub exam_date: String,           // 3 EXAM DATE
    pub procedure: String,           // 4 PROCEDURE
    pub status: Option<String>,      // 5 STATUS
    pub interpreting_physician: Option<String>, // 6 INTERPRETING PHYSICIAN
}
