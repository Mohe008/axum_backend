use axum::{Json, extract::State};
use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait, Set, TransactionTrait};

use crate::entities::{enrollments, payments, prelude::*};

use crate::error::AppError;
use crate::models::{ CreateEnrollment, EnrollmentResult };

pub async fn get_enrollments(
    State(db): State<DatabaseConnection>
) -> Result<Json<Vec<enrollments::Model>>, AppError> {
    let enrollments = Enrollments::find()
        .all(&db)
        .await?;

    Ok(Json(enrollments))
}

pub async fn create_enrollments(
    State(db): State<DatabaseConnection>,
    Json(payload): Json<CreateEnrollment>
) -> Result<Json<EnrollmentResult>, AppError>{
    let course = Courses::find_by_id(payload.course_id)
        .one(&db)
        .await?
        .ok_or(AppError::NotFound("Course not found".to_string()))?;

    let txn = db.begin().await?;

    let new_enrollment = enrollments::ActiveModel {
        student_id: Set(Some(payload.student_id)),
        course_id: Set(Some(payload.course_id)),
        ..Default::default()
    };
    let enrollment = new_enrollment.insert(&db).await?;

    let new_payment = payments::ActiveModel {
        student_id: Set(payload.student_id),
        amount: Set(course.fee),
        status: Set("pending".to_string()),
        ..Default::default()
    };
    let payment = new_payment.insert(&db).await?;

    txn.commit().await?;

    Ok(Json(EnrollmentResult { enrollment, payment}))
}