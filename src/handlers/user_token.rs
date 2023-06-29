use actix_web::{http::StatusCode, web, HttpRequest, HttpResponse, Error};

use crate::{MyData, dao::sys_model_request};

use super::get_auth_token;

pub async fn weekly_tokens(data: web::Data<MyData>, req: HttpRequest) -> Result<HttpResponse, Error> {
    let auth = get_auth_token(req).unwrap();
    Ok(HttpResponse::build(StatusCode::OK).json(sys_model_request::get_week_tokens(&data.pool, auth).await))
}