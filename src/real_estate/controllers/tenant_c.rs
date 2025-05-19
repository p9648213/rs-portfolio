use axum::{Extension, extract::Path, http::HeaderMap, response::Html};
use reqwest::StatusCode;
use vy::IntoHtml;

use crate::real_estate::views::pages::tenant_v::render_tenant_page;
use crate::{
    common::{middlewares::auth_mw::UserAuth, models::error::AppError},
    real_estate::views::pages::tenant_v::{TenantPageProps, TenantSlug, render_tentant_content},
};

pub async fn get_tenant_page(
    Path(slug): Path<String>,
    Extension(user_auth): Extension<UserAuth>,
    headers: HeaderMap,
) -> Result<Html<String>, AppError> {
    if let Some(user_info) = user_auth.0 {
        if user_info.rs_role != "tenant" {
            return Err(AppError::new(StatusCode::FORBIDDEN, "Forbidden"));
        }

        let boosted = headers.get("HX-Boosted");

        if boosted.is_some() {
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

                return Ok(Html(render_tentant_content(&slug).into_string()));
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

        Ok(Html(render_tenant_page(&props).into_string()))
    } else {
        Err(AppError::new(StatusCode::UNAUTHORIZED, "Unauthorized"))
    }
}
