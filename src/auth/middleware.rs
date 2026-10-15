use axum::{RequestPartsExt, extract::FromRequestParts, http::{StatusCode, request::Parts}};
use axum_extra::{TypedHeader, headers::{Authorization, authorization::Bearer}};

use crate::auth::jwt::verify_token;

pub struct AuthUser {
    pub user_id: i32,
    pub role: String
}

impl<S> FromRequestParts<S> for AuthUser
where
    S: Send + Sync
{
    type Rejection = (StatusCode, String);

    async fn from_request_parts(
        parts: &mut Parts,
        _state: &S,
    ) -> Result<Self, Self::Rejection>
    {
        let TypedHeader(Authorization(bearer)) = parts
            .extract::<TypedHeader<Authorization<Bearer>>>()
            .await
            .map_err(|_|{
                (
                    StatusCode::UNAUTHORIZED,
                    "Missing or invalid Authorization header".to_string()
                )
            })?;

        let claims = verify_token(bearer.token())
            .map_err(|_| (StatusCode::UNAUTHORIZED, "Invalid or expired token".to_string()))?;

        Ok(AuthUser {
            user_id: claims.sub,
            role: claims.role
        })
    }
}


pub struct AdminUser {
    pub user_id: i32
}

impl<S> FromRequestParts<S> for AdminUser
where 
    S: Send + Sync,
{
    type Rejection = (StatusCode, String);

    async fn from_request_parts(
        parts: &mut Parts,
        state: &S,
    ) -> Result<Self, Self::Rejection>
    {
        let auth_user = AuthUser::from_request_parts(parts, state).await?;

        if auth_user.role != "admin" {
            return Err((
                StatusCode::FORBIDDEN,
                "Admin access required".to_string()
            ));
        };

        Ok(AdminUser { user_id: auth_user.user_id })
    }
}