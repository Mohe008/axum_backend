use axum::{Json, extract::{Path, State}};
use chrono::Utc;
use sea_orm::{ActiveModelTrait, Set, DatabaseConnection, EntityTrait};

use validator::Validate;
use crate::entities::{payments, prelude::* };
use crate::auth::middleware::AdminUser;
use crate::error::AppError;

use crate::models::UpdatePaymentStatus;

pub async fn update_payment_status(
    State(db): State<DatabaseConnection>,
    _admin: AdminUser,
    Path(id): Path<i32>,
    Json(payload): Json<UpdatePaymentStatus>
) -> Result<Json<payments::Model>, AppError> {
    payload.validate()?;

    let payment = Payments::find_by_id(id)
        .one(&db)
        .await?
        .ok_or(AppError::NotFound("Payment not found".to_string()))?;

    let mut payment: payments::ActiveModel = payment.into();

    payment.status = Set(payload.status.clone());

    if payload.status == "completed" {
        payment.paid_at = Set(Some(Utc::now().naive_utc()))
    }
    let updated = payment.update(&db).await?;

    Ok(Json(updated))
}

pub async fn get_payments(
    State(db): State<DatabaseConnection>
) -> Result<Json<Vec<payments::Model>>, AppError>{
    let result = Payments::find()
        .all(&db)
        .await?;

    Ok(Json(result))
}