use actix_web::{HttpRequest, Error};

pub mod index;
pub mod parts;
pub mod products;
pub mod chat;
pub mod user_token;

/// 从请求头中获取 Bearer token
pub fn get_auth_token(req: HttpRequest) -> Result<String, Error> {
    let auth = req.headers()
        .get("Authorization")
        .and_then(|d| Some(d.to_str()))
        .ok_or_else(|| actix_web::error::ErrorUnauthorized("dont have token"))?
        .map_err(|_| actix_web::error::ErrorUnauthorized("token error"))?;
    if !auth.starts_with("Bearer") || auth.len() < "Bearer".len() + 5 {
        return Err(actix_web::error::ErrorUnauthorized("not a valid bearer token"));
    }
    Ok(auth["Bearer".len()..].trim().to_string())
}