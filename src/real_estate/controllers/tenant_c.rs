use axum::{Extension, extract::Path, response::Html};
use reqwest::StatusCode;
use vy::IntoHtml;

use crate::{
    common::{middlewares::auth_mw::UserAuth, models::error::AppError},
    real_estate::views::pages::tenant_v::{TenantPageProps, TenantSlug, render_tenant_page},
};

pub async fn get_tenant_page(
    Path(slug): Path<String>,
    Extension(user_auth): Extension<UserAuth>,
) -> Result<Html<String>, AppError> {
    if let Some(user_info) = user_auth.0 {
        if user_info.rs_role != "tenant" {
            return Err(AppError::new(StatusCode::FORBIDDEN, "Forbidden"));
        }

        let props = TenantPageProps {
            user_info,
            is_dashboard_page: true,
            slug: match slug.as_str() {
                "favorites" => TenantSlug::Favorites,
                _ => return Err(AppError::new(StatusCode::NOT_FOUND, "Not found")),
            },
        };

        Ok(Html(render_tenant_page(&props).into_string()))
    } else {
        Err(AppError::new(StatusCode::UNAUTHORIZED, "Unauthorized"))
    }
}
