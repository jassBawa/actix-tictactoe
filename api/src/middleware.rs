use actix_web::{
    Error, HttpMessage, HttpRequest,
    body::BoxBody,
    dev::{ServiceRequest, ServiceResponse},
    error::ErrorUnauthorized,
};
use actix_web_lab::middleware::Next;
use uuid::Uuid;

use crate::utils::verify_jwt;

#[derive(Clone, Debug)]
pub struct AuthenticatedUser {
    pub user_id: Uuid,
}

pub fn extract_user_from_request(req: &HttpRequest) -> Result<Uuid, Error> {
    let auth_header = req
        .headers()
        .get("Authorization")
        .ok_or_else(|| ErrorUnauthorized("Missing authorization header"))?;

    let header_str = auth_header
        .to_str()
        .map_err(|_| ErrorUnauthorized("Invalid authorization header"))?;

    let token = if header_str.starts_with("Bearer ") {
        header_str[7..].to_string()
    } else {
        header_str.to_string()
    };

    let user_id = verify_jwt(&token).map_err(|_| ErrorUnauthorized("Invalid or expired token"))?;

    Ok(user_id)
}

pub async fn jwt_auth_fn(
    req: ServiceRequest,
    next: Next<BoxBody>,
) -> Result<ServiceResponse<BoxBody>, Error> {
    let user_id = extract_user_from_request(req.request())?;
    req.extensions_mut().insert(AuthenticatedUser { user_id });
    next.call(req).await
}
