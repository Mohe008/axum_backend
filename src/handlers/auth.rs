use axum::{Json, extract::State};
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};
use serde::{Deserialize, Serialize};

use crate::auth::jwt::generate_token;
use crate::auth::password::verify_password;
use crate::entities::{ users, prelude::* };
use crate::error::AppError;

#[derive(Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String
}

#[derive(Serialize)]
pub struct LoginResponse {
    pub token: String,
    pub user: users::Model
}

pub async fn login(
    State(db): State<DatabaseConnection>,
    Json(payload): Json<LoginRequest>
) -> Result<Json<LoginResponse>, AppError> {
    let user = Users::find()
        .filter(users::Column::Email.eq(payload.email))
        .one(&db)
        .await?
        .ok_or(AppError::NotFound("User not found".to_string()))?;

    let is_valid = verify_password(&payload.password, &user.password);
    if !is_valid {
        return Err(AppError::NotFound("Invalid email or password".to_string()));
    }

    let token = generate_token(user.id, &user.role)
        .map_err(AppError::ValidationError)?;

    Ok(Json(LoginResponse { token, user }))
}
