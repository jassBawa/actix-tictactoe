use actix_web::{
    Error, HttpMessage,
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

pub async fn jwt_auth_fn(
    req: ServiceRequest,
    next: Next<BoxBody>,
) -> Result<ServiceResponse<BoxBody>, Error> {
    // Get the Authorization header
    let auth_header = req.headers().get("Authorization");

    let token = match auth_header {
        Some(header_value) => {
            let header_str = header_value
                .to_str()
                .map_err(|_| ErrorUnauthorized("Invalid authorization header"))?;

            if header_str.starts_with("Bearer ") {
                header_str[7..].to_string()
            } else {
                header_str.to_string()
            }
        }
        None => return Err(ErrorUnauthorized("Missing authorization header")),
    };

    let user_id = verify_jwt(&token).map_err(|_| ErrorUnauthorized("Invalid or expired token"))?;

    req.extensions_mut().insert(AuthenticatedUser { user_id });

    next.call(req).await
}
