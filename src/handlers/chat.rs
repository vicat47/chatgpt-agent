use actix_web::{error, HttpRequest, web, Error, HttpResponse};

use crate::{dao::{sys_user_token::get_by_local_token, sys_model_request::save_token}, MyData, OpenAiChat};

pub async fn chat_to(data: web::Data<MyData>, req: HttpRequest, body: web::Bytes) -> Result<HttpResponse, Error> {
    log::debug!("{req:?}");
    // println!("{content:?}");

    // 认证
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

    // 获取 token
    let user = get_by_local_token(&data.pool, auth.to_string()).await;

    let user = match user {
        Some(user) => user,
        None => panic!("no such user"),
    };

    let _token = user.gpt_token;

    let body = &serde_json::from_str::<serde_json::Value>(std::str::from_utf8(&body).expect("json data error...")).expect("not a json");
    // let body = json::parse(std::str::from_utf8(&body).unwrap()).expect("body is not a json");

    log::debug!("receved user send request: {body:#?}");

    let res = data.client.post(data.target_url.clone())
        .bearer_auth(_token)
        .json(body)
        .send()
        .await
        .map_err(error::ErrorInternalServerError)?;

    let mut client_resp = HttpResponse::build(res.status());

    let json: serde_json::Value = res.json().await.expect("deserilize json error, openai returns invalid json");
    
    log::debug!("{json:#?}");
    let chat: OpenAiChat = match serde_json::from_value(json.clone()) {
        Ok(j) => j,
        Err(e) => {
            log::error!("{e}");
            return Ok(client_resp.json(json));
        },
    };
    log::debug!("{chat:#?}");

    save_token(&data.pool, chat.id, chat.model, chat.usage).await;

    Ok(client_resp.json(json))
}