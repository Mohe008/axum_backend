use axum::{Router, routing::{get, patch, post}};
use sea_orm::DatabaseConnection;
use tower_http::{cors::{Any, CorsLayer}, trace::{DefaultMakeSpan, DefaultOnResponse, TraceLayer}};
use axum::http::Method;
use tracing::Level;


use crate::handlers::{
    auth::login, courses::{get_course_students, get_courses}, enrollments::{create_enrollments, get_enrollments}, payments::{ get_payments, update_payment_status }, users::{
        create_user,
        delete_user,
        get_paginated_users,
        get_user,
        get_user_courses,
        get_user_with_department,
        get_users,
        search_user,
        update_user
}}; 

pub fn app(db: DatabaseConnection) -> Router {
    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST, Method::PATCH, Method::DELETE])
        .allow_origin(Any)
        .allow_headers(Any);

    Router::new()
        .route("/", get(|| async { "hello_world" }))
        .route("/users", get(get_users).post(create_user))
        .route("/users/{id}", get(get_user).patch(update_user).delete(delete_user))
        .route("/get-paginated-users", get(get_paginated_users))
        .route("/search-user", get(search_user))
        .route("/users/{id}/department", get(get_user_with_department))
        .route("/users/{id}/courses", get(get_user_courses))
        .route("/courses", get(get_courses))
        .route("/courses/{id}/students", get(get_course_students))
        .route("/enrollments",  get(get_enrollments).post(create_enrollments))
        .route("/payments", get(get_payments))
        .route("/payments/{id}/status", patch(update_payment_status))
        .route("/login", post(login))
        .layer(cors)
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(DefaultMakeSpan::new().level(Level::INFO))
                .on_response(DefaultOnResponse::new().level(Level::INFO))
        )
        .with_state(db)
}
