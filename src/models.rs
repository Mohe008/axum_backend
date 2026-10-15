use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationError};

use crate::entities::{departments, enrollments, payments, users};

fn validate_role(role: &str) -> Result<(), ValidationError> {
    match role {
        "student" | "professor" | "admin" => Ok(()),
        _ => Err(ValidationError::new("Invalid Role"))
    }
}

#[derive(Deserialize, Validate)]
pub struct CreateUser {
    #[validate(length(min=2, message="Name cannot be empty"))]
    pub name: String,

    #[validate(email(message="Invalid email format"))]
    pub email: String,

    #[validate(length(min = 8, message = "Password must be at least 8 characters"))]
    pub password: String,

    #[validate(custom(function = "validate_role", message = "Role must be either 'student' or 'professor'"))]
    pub role: String,
    pub department_id: Option<i32>,
    pub phone_number: Option<String>
}

#[derive(Deserialize, Validate)]
pub struct UpdateUser {
    #[validate(length(min=2, message="Name cannot be empty"))]
    pub name: Option<String>,
    #[validate(email(message="Invalid email format"))]
    pub email: Option<String>,
    #[validate(custom(function = "validate_role", message = "Role must be either 'student' or 'professor'"))]
    pub role: Option<String>,
    pub is_active: Option<bool>,
    pub department_id: Option<i32>,
    pub phone_number: Option<String>
}

#[derive(Serialize)]
pub struct UserWithDepartment {
    #[serde(flatten)]
    pub user: users::Model,
    pub department: Option<departments::Model>
}

#[derive(Deserialize)]
pub struct PaginationParams {
    pub page: Option<u64>,
    pub per_page: Option<u64>
}

#[derive(Serialize)]
pub struct PaginatedUsers {
    pub data: Vec<users::Model>,
    pub page: u64,
    pub per_page: u64,
    pub total_pages: u64,
    pub total_items: u64
}

#[derive(Deserialize)]
pub struct SearchParams {
    pub role: Option<String>,
    pub is_active: Option<bool>,
    pub department_id: Option<i32>,
    pub search: Option<String>
}

#[derive(Deserialize)]
pub struct CreateEnrollment {
    pub student_id: i32,
    pub course_id: i32
}

#[derive(Serialize)]
pub struct EnrollmentResult {
    pub enrollment: enrollments::Model,
    pub payment: payments::Model
}


fn validate_status(status: &str) -> Result<(), ValidationError> {
    match status {
        "pending" | "completed" | "failed" => Ok(()),
        _ => Err(ValidationError::new("Invalid status!!"))
    }
}
#[derive(Deserialize, Validate)]
pub struct UpdatePaymentStatus {
    #[validate(custom(function = "validate_status", message = "Status must be one of: pending, completed, failed"))]
    pub status: String
}


