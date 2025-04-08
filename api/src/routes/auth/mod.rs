use actix_web::{post, web, HttpResponse, Responder};
use bcrypt::{hash, verify, DEFAULT_COST};
use jsonwebtoken::{encode, EncodingKey, Header};
use chrono::{Utc, Duration};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

use crate::models::{CreateUserRequest, LoginRequest, LoginResponse, User, UserResponse};
use crate::errors::ApiError;
use crate::config::Config;

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: i32,
    username: String,
    exp: i64,
}

#[post("/register")]
async fn register(
    pool: web::Data<PgPool>,
    user_data: web::Json<CreateUserRequest>,
) -> Result<impl Responder, ApiError> {
    // Check if user already exists
    let existing_user = sqlx::query!(
        r#"SELECT id FROM pbtar.users WHERE username = $1 OR email = $2"#,
        user_data.username,
        user_data.email
    )
    .fetch_optional(pool.get_ref())
    .await
    .map_err(|e| ApiError::DbError(e))?;

    if existing_user.is_some() {
        return Err(ApiError::BadRequestError("Username or email already exists".into()));
    }

    // Hash the password
    let password_hash = hash(&user_data.password, DEFAULT_COST)
        .map_err(|_| ApiError::InternalError("Failed to hash password".into()))?;

    // Insert the new user
    let user = sqlx::query_as!(
        User,
        r#"
        INSERT INTO pbtar.users (username, email, password_hash)
        VALUES ($1, $2, $3)
        RETURNING id, username, email, password_hash, created_at, updated_at
        "#,
        user_data.username,
        user_data.email,
        password_hash
    )
    .fetch_one(pool.get_ref())
    .await
    .map_err(|e| ApiError::DbError(e))?;

    // Create response
    let user_response = UserResponse {
        id: user.id,
        username: user.username,
        email: user.email,
    };

    Ok(HttpResponse::Created().json(user_response))
}

#[post("/login")]
async fn login(
    pool: web::Data<PgPool>,
    login_data: web::Json<LoginRequest>,
    config: web::Data<Config>,
) -> Result<impl Responder, ApiError> {
    // Find the user
    let user = sqlx::query_as!(
        User,
        r#"SELECT * FROM pbtar.users WHERE username = $1"#,
        login_data.username
    )
    .fetch_optional(pool.get_ref())
    .await
    .map_err(|e| ApiError::DbError(e))?
    .ok_or_else(|| ApiError::AuthError("Invalid username or password".into()))?;

    // Verify password
    let is_valid = verify(&login_data.password, &user.password_hash)
        .map_err(|_| ApiError::AuthError("Invalid username or password".into()))?;

    if !is_valid {
        return Err(ApiError::AuthError("Invalid username or password".into()));
    }

    // Generate JWT
    let expiration = Utc::now()
        .checked_add_signed(Duration::seconds(config.jwt_expiration))
        .expect("Valid timestamp")
        .timestamp();

    let claims = Claims {
        sub: user.id,
        username: user.username.clone(),
        exp: expiration,
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(config.jwt_secret.as_bytes()),
    )
    .map_err(|_| ApiError::InternalError("Failed to generate token".into()))?;

    // Create response
    let response = LoginResponse {
        token,
        user: UserResponse {
            id: user.id,
            username: user.username,
            email: user.email,
        },
    };

    Ok(HttpResponse::Ok().json(response))
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/auth")
        .service(register)
        .service(login)
    );
}