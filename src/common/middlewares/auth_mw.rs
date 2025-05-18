use axum::{extract::Request, middleware::Next, response::IntoResponse};
use axum_session::Session;
use axum_session_redispool::SessionRedisPool;

use crate::common::models::error::AppError;

#[derive(Clone, Debug)]
pub struct UserInfo {
    pub id: String,
    pub username: String,
    pub image_url: String,
    pub rs_role: String,
}

#[derive(Clone, Debug)]
pub struct UserAuth(pub Option<UserInfo>);

pub async fn auth_middleware(
    session: Session<SessionRedisPool>,
    mut request: Request,
    next: Next,
) -> Result<impl IntoResponse, AppError> {
    let user_id: Option<String> = session.get("user-id");

    if let Some(id) = user_id {
        let username: Option<String> = session.get("user-name");
        let image_url: Option<String> = session.get("user-image");
        let rs_role: Option<String> = session.get("user-rs-role");

        let user_info = UserInfo {
            id,
            username: username.unwrap_or_default(),
            image_url: image_url.unwrap_or_default(),
            rs_role: rs_role.unwrap_or_default(),
        };

        request.extensions_mut().insert(UserAuth(Some(user_info)));

        Ok(next.run(request).await.into_response())
    } else {
        request.extensions_mut().insert(UserAuth(None));
        Ok(next.run(request).await.into_response())
    }
}
