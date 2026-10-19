use axum::{
    Json, extract::{Path, Query, State}, http::StatusCode,
};
use sea_orm::{ActiveModelTrait, ColumnTrait, Condition, DatabaseConnection, EntityTrait, ModelTrait, PaginatorTrait, QueryFilter, Set, sea_query::Expr};
use validator::Validate;

use crate::{entities::{courses, prelude::*, users}, links::UserCourses};
use crate::auth::password::hash_password;
use crate::auth::middleware::AdminUser;
use crate::error::AppError;

use crate::models::{
    CreateUser,
    UpdateUser,
    UserWithDepartment,
    PaginatedUsers,
    PaginationParams,
    SearchParams
};

pub async fn get_users(
    State(db): State<DatabaseConnection>,
) -> Result<Json<Vec<users::Model>>, AppError> {
    let users = Users::find()
        .all(&db)
        .await?;

    Ok(Json(users))
}

pub async fn get_user(
    State(db): State<DatabaseConnection>,
    Path(id): Path<i32>,
) -> Result<Json<users::Model>, AppError> {
    let user = Users::find_by_id(id)
        .one(&db)
        .await?
        .ok_or(AppError::NotFound("User not found".to_string()))?;

    Ok(Json(user))
}

pub async fn create_user(
    State(db): State<DatabaseConnection>,
    Json(payload): Json<CreateUser>
) -> Result<Json<users::Model>, AppError> {
    payload.validate()?;

    let password = hash_password(&payload.password)
        .map_err(|e| AppError::ValidationError(e))?;

    let new_user = users::ActiveModel {
        name: Set(payload.name),
        email: Set(payload.email),
        password: Set(password),
        role: Set(payload.role),
        metadata: Set(payload.metadata),
        department_id: Set(payload.department_id),
        phone_number: Set(payload.phone_number),
        ..Default::default()
    };

    let result = new_user
        .insert(&db)
        .await?;

    Ok(Json(result))
}

pub async fn update_user(
    State(db): State<DatabaseConnection>,
    Path(id): Path<i32>,
    Json(payload): Json<UpdateUser>
) -> Result<Json<users::Model>, AppError> {
    payload.validate()?;

    let user = Users::find_by_id(id)
        .one(&db)
        .await?
        .ok_or(AppError::NotFound("User not found".to_string()))?;

    let mut user: users::ActiveModel = user.into();

    if let Some(name) = payload.name {
        user.name = Set(name);
    }
    if let Some(email) = payload.email {
        user.email = Set(email);
    }
    if let Some(role) = payload.role {
        user.role = Set(role);
    }
    if let Some(is_active) = payload.is_active {
        user.is_active = Set(is_active);
    }
    if let Some(metadata) = payload.metadata {
        user.metadata = Set(Some(metadata));
    }
    if let Some(department_id) = payload.department_id {
        user.department_id = Set(Some(department_id));
    }
    if let Some(phone_number) = payload.phone_number {
        user.phone_number = Set(Some(phone_number));
    }

    let updated = user
        .update(&db)
        .await?;

    Ok(Json(updated))
}

pub async fn delete_user(
    State(db): State<DatabaseConnection>,
    _admin: AdminUser,
    Path(id): Path<i32>
) -> Result<StatusCode, AppError> {
    let result = Users::delete_by_id(id)
        .exec(&db)
        .await?;

    if result.rows_affected == 0 {
        return Err(AppError::NotFound("User not found".to_string()));
    }

    Ok(StatusCode::NO_CONTENT)
}

pub async fn get_user_with_department(
    State(db): State<DatabaseConnection>,
    Path(id): Path<i32>
) -> Result<Json<UserWithDepartment>, AppError> {
    let result = Users::find_by_id(id)
        .find_also_related(Departments)
        .one(&db)
        .await?
        .ok_or(AppError::NotFound("User not found".to_string()))?;

    let (user, department) = result;
    Ok(Json(UserWithDepartment { user, department }))
}

pub async fn get_paginated_users(
    State(db): State<DatabaseConnection>,
    Query(params): Query<PaginationParams>
) -> Result<Json<PaginatedUsers>, AppError> {
    let page = params.page.unwrap_or(1).max(1);
    let per_page = params.per_page.unwrap_or(10).clamp(1, 100);

    let paginator = Users::find()
        .order_by_id_asc()
        .paginate(&db, per_page);

    let total_pages = paginator
        .num_pages()
        .await?;
    let total_items = paginator
        .num_items()
        .await?;

    let data = paginator
        .fetch_page(page - 1)
        .await?;

    Ok(Json(PaginatedUsers{
        data,
        page,
        per_page,
        total_pages,
        total_items
    }))
}

pub async fn search_user(
    State(db): State<DatabaseConnection>,
    Query(params): Query<SearchParams>
) -> Result<Json<Vec<users::Model>>, AppError> {
    let mut condition = Condition::all();

    if let Some(role) = params.role {
        condition = condition.add(users::Column::Role.eq(role));
    }
    if let Some(is_active) = params.is_active {
        condition = condition.add(users::Column::IsActive.eq(is_active));
    }
    if let Some(department_id) = params.department_id {
        condition = condition.add(users::Column::DepartmentId.eq(department_id));
    }

    if let Some(search) = params.search {
        let pattern = format!("%{}%", search);

        condition = condition.add(
            Condition::any()
            .add(users::Column::Name.ilike(&pattern))
            .add(users::Column::Email.ilike(&pattern)),
        );
    }

    if let Some(min_cgpa) = params.min_cgpa {
        condition = condition.add(
            Expr::cust_with_values(
                "(metadata->>'cgpa')::float >= $1",
                [min_cgpa]
            )
        );
    }

    let data = Users::find()
        .filter(condition)
        .all(&db)
        .await?;
        
    
    Ok(Json(data))
}

pub async fn get_user_courses(
    State(db): State<DatabaseConnection>,
    Path(id): Path<i32>
) -> Result<Json<Vec<courses::Model>>, AppError>{
    let user = Users::find_by_id(id)
        .one(&db)
        .await?
        .ok_or(AppError::NotFound("User not found".to_string()))?;

    let courses = user.find_linked(UserCourses).all(&db).await?;

    Ok(Json(courses))
}

