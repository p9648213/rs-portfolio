use axum::{Extension, Form, extract::State, response::IntoResponse};
use axum_session::Session;
use axum_session_redispool::SessionRedisPool;
use deadpool_postgres::Pool;
use reqwest::StatusCode;
use serde::Deserialize;

use crate::common::{
    middlewares::auth_mw::UserAuth,
    models::{error::AppError, user_db::User},
};

#[derive(Deserialize, Debug)]
pub struct UpdateUserForm {
    pub username: String,
    pub phone_number: String,
}

pub async fn update_user(
    session: Session<SessionRedisPool>,
    Extension(user_auth): Extension<UserAuth>,
    State(pg_pool): State<Pool>,
    Form(update_form): Form<UpdateUserForm>,
) -> Result<impl IntoResponse, AppError> {
    if update_form.username.is_empty() || update_form.phone_number.is_empty() {
        return Err(AppError::new(
            StatusCode::BAD_REQUEST,
            "Input can not be empty",
        ));
    }

    let user_info = user_auth
        .0
        .ok_or(AppError::new(StatusCode::UNAUTHORIZED, "Unauthorized"))?;

    let result = User::update_realestate_user(
        &user_info.id,
        &update_form.username,
        &update_form.phone_number,
        &pg_pool,
    )
    .await?;

    if result > 0 {
        session.set("user-name", &update_form.username);
    }

    Ok([(
        "HX-Trigger",
        format!(
            "{{\
                \"toastmessage\":{{\"type\":\"success\",\"message\":\"User update successfully\"}},\
                \"update_user\":{{\"type\":\"success\",\"username\":\"{}\"}}\
            }}",
            update_form.username
        ),
    )])
}
