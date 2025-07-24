use axum::{extract::{Path, State}, Json};
use axum::http::StatusCode;
use sqlx::sqlite::SqlitePool;
use std::sync::Arc;
use crate::dto::patient_dto::{PatientInput, PatientOut};

pub async fn list_patients(State(pool): State<Arc<SqlitePool>>) -> Json<Vec<PatientOut>> {
    let rows = sqlx::query_as!(
        PatientOut,
        r#"SELECT id, name, sex, date_of_birth, ssn, marital_status, religion, address_line1, address_line2, address_line3, city, state, zip, county, phone_home, phone_work, next_of_kin, next_of_kin_phone, emergency_contact, emergency_contact_phone FROM patient ORDER BY id"#
    )
    .fetch_all(&*pool)
    .await
    .unwrap_or_default();
    Json(rows)
}

pub async fn get_patient(State(pool): State<Arc<SqlitePool>>, Path(id): Path<i64>) -> Option<Json<PatientOut>> {
    let row = sqlx::query_as!(
        PatientOut,
        r#"SELECT id, name, sex, date_of_birth, ssn, marital_status, religion, address_line1, address_line2, address_line3, city, state, zip, county, phone_home, phone_work, next_of_kin, next_of_kin_phone, emergency_contact, emergency_contact_phone FROM patient WHERE id = ?"#,
        id
    )
    .fetch_optional(&*pool)
    .await
    .ok()?;
    row.map(Json)
}

pub async fn create_patient(State(pool): State<Arc<SqlitePool>>, Json(input): Json<PatientInput>) -> Json<i64> {
    let rec = sqlx::query!(
        r#"INSERT INTO patient (name, sex, date_of_birth, ssn, marital_status, religion, address_line1, address_line2, address_line3, city, state, zip, county, phone_home, phone_work, next_of_kin, next_of_kin_phone, emergency_contact, emergency_contact_phone)
        VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
        input.name,
        input.sex,
        input.date_of_birth,
        input.ssn,
        input.marital_status,
        input.religion,
        input.address_line1,
        input.address_line2,
        input.address_line3,
        input.city,
        input.state,
        input.zip,
        input.county,
        input.phone_home,
        input.phone_work,
        input.next_of_kin,
        input.next_of_kin_phone,
        input.emergency_contact,
        input.emergency_contact_phone
    )
    .execute(&*pool)
    .await
    .unwrap();
    Json(rec.last_insert_rowid())
}

pub async fn update_patient(State(pool): State<Arc<SqlitePool>>, Path(id): Path<i64>, Json(input): Json<PatientInput>) -> StatusCode {
    let res = sqlx::query!(
        r#"UPDATE patient SET name=?, sex=?, date_of_birth=?, ssn=?, marital_status=?, religion=?, address_line1=?, address_line2=?, address_line3=?, city=?, state=?, zip=?, county=?, phone_home=?, phone_work=?, next_of_kin=?, next_of_kin_phone=?, emergency_contact=?, emergency_contact_phone=? WHERE id=?"#,
        input.name,
        input.sex,
        input.date_of_birth,
        input.ssn,
        input.marital_status,
        input.religion,
        input.address_line1,
        input.address_line2,
        input.address_line3,
        input.city,
        input.state,
        input.zip,
        input.county,
        input.phone_home,
        input.phone_work,
        input.next_of_kin,
        input.next_of_kin_phone,
        input.emergency_contact,
        input.emergency_contact_phone,
        id
    )
    .execute(&*pool)
    .await;
    match res {
        Ok(_) => StatusCode::NO_CONTENT,
        Err(_) => StatusCode::NOT_FOUND,
    }
}

pub async fn delete_patient(State(pool): State<Arc<SqlitePool>>, Path(id): Path<i64>) -> StatusCode {
    let res = sqlx::query!("DELETE FROM patient WHERE id = ?", id)
        .execute(&*pool)
        .await;
    match res {
        Ok(_) => StatusCode::NO_CONTENT,
        Err(_) => StatusCode::NOT_FOUND,
    }
}
