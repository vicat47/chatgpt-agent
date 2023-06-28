use actix_web::{HttpRequest, web, Error, HttpResponse};
use serde_json::{Value};
use sqlx::PgPool;

use crate::dao::sys_user_token::get_by_local_token;

pub async fn chat_to(data: web::Data<PgPool>, req: HttpRequest, content: web::Json<Value>) -> Result<HttpResponse, Error> {
    println!("{req:?}");
    let auth = match req.headers().get("Authorization").map(| d | d.to_str()) {
        Some(data) => match data {
            Ok(data) => data,
            Err(_) => panic!("no authorizaion"),
        },
        None => panic!("no authorization"),
    };
    assert!(auth.starts_with("Bearer"), "not a valid bearer token");
    assert!(auth.len() > "Bearer".len() + 5, "not long enough");

    let auth = auth["Bearer".len()..].trim();

    let user = get_by_local_token(&data, auth.to_string()).await;

    let user = match user {
        Some(user) => user,
        None => panic!("no such user"),
    };

    let _token = user.gpt_token;

    println!("{:?}", _token);

    Ok(HttpResponse::Ok().finish())
}