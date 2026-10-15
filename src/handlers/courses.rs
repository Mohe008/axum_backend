use axum::{Json, extract::{Path, State}};
use sea_orm::{DatabaseConnection, EntityTrait, ModelTrait};

use crate::{entities::{courses, prelude::*, users}, links::CourseUsers};
use crate::error::AppError;


pub async fn get_course_students(
    State(db): State<DatabaseConnection>,
    Path(id): Path<i32>
) -> Result<Json<Vec<users::Model>>, AppError> {
    let course = Courses::find_by_id(id)
        .one(&db)
        .await?
        .ok_or(AppError::NotFound("Course not found".to_string()))?;

    let students = course.find_linked(CourseUsers).all(&db).await?;

    Ok(Json(students))
}

pub async fn get_courses(
    State(db): State<DatabaseConnection>
) -> Result<Json<Vec<courses::Model>>, AppError> {
    let courses = Courses::find()
        .all(&db)
        .await?;

    Ok(Json(courses))
}