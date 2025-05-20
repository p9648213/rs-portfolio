use axum::extract::State;
use axum::response::{IntoResponse, Response};
use axum::{Extension, extract::Path, http::HeaderMap, response::Html};
use deadpool_postgres::Pool;
use reqwest::StatusCode;
use vy::IntoHtml;

use crate::common::models::user_db::User;
use crate::real_estate::views::pages::tenant_v::{render_tenant_page, render_tenant_section};
use crate::real_estate::views::ui::tenant::settings_v::render_settings_form;
use crate::{
    common::{middlewares::auth_mw::UserAuth, models::error::AppError},
    real_estate::views::pages::tenant_v::{TenantPageProps, TenantSlug},
};

pub async fn get_tenant_page(
    Path(slug): Path<String>,
    Extension(user_auth): Extension<UserAuth>,
    headers: HeaderMap,
) -> Result<Response, AppError> {
    if let Some(user_info) = user_auth.0 {
        if user_info.rs_role != "tenant" {
            return Err(AppError::new(StatusCode::FORBIDDEN, "Forbidden"));
        }

        let boosted = headers.get("HX-Boosted");
        let target = headers.get("HX-Target");

        if boosted.is_some() && target.is_some() {
            let referer = headers.get(reqwest::header::REFERER);

            if referer.is_some()
                && referer
                    .unwrap()
                    .to_str()
                    .unwrap_or_default()
                    .contains("/tenant")
            {
                let slug = match slug.as_str() {
                    "favorites" => TenantSlug::Favorites,
                    "settings" => TenantSlug::Settings,
                    _ => return Err(AppError::new(StatusCode::NOT_FOUND, "Not found")),
                };

                return Ok(Html(render_tenant_section(&slug).into_string()).into_response());
            }
        }

        let props = TenantPageProps {
            user_info: &user_info,
            is_dashboard_page: true,
            slug: match slug.as_str() {
                "favorites" => TenantSlug::Favorites,
                "settings" => TenantSlug::Settings,
                _ => return Err(AppError::new(StatusCode::NOT_FOUND, "Not found")),
            },
        };

        Ok(Html(render_tenant_page(&props).into_string()).into_response())
    } else {
        let response = Response::builder()
            .status(StatusCode::OK)
            .header("HX-Redirect", "/realestate")
            .body(axum::body::Body::empty())
            .unwrap();

        Ok(response)
    }
}

pub async fn get_tenant_ui(
    Path(slug): Path<String>,
    Extension(user_auth): Extension<UserAuth>,
    State(pg_pool): State<Pool>,
) -> Result<Html<String>, AppError> {
    let user_info = user_auth
        .0
        .ok_or(AppError::new(StatusCode::UNAUTHORIZED, "Unauthorized"))?;

    match slug.as_str() {
        "settings-form" => {
            let row =
                User::get_user_by_id(&user_info.id, &pg_pool, vec!["username", "phone_number"])
                    .await?
                    .ok_or_else(|| {
                        tracing::error!("Error getting user by id: {}", user_info.id);
                        AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Server Error")
                    })?;

            let user = User::try_from(&row, None);

            let user_name = user.username.ok_or_else(|| {
                tracing::error!("No username column or value is null");
                AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Server error")
            })?;

            let phone_number = user.phone_number.unwrap_or_default();

            Ok(Html(
                render_settings_form(&user_name, &phone_number).into_string(),
            ))
        }
        _ => Err(AppError::new(StatusCode::NOT_FOUND, "Not found")),
    }
}
