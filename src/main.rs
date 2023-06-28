use std::env;

use actix_web::{middleware, App, HttpServer, web};
use chatgpt_agent::{app_config::config_app, MyData};
use sqlx::{postgres::PgPoolOptions, types::chrono::{DateTime, Local}};
use dotenv::dotenv;
use url::Url;

#[derive(Debug, sqlx::FromRow)]
pub struct SysUserToken {
    pub id: i32,
    pub name: String,
    pub local_token: String,
    pub gpt_token: String,
    pub create_time: DateTime<Local>, 
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("debug"));

    log::info!("starting HTTP server at http://localhost:8080");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&env::var("DATABASE_URL").expect(r#"environment variable "DATABASE_URL" not exists"#)).await.unwrap();

    let openai_url = env::var("OPENAI_URL").expect(r#"environment variable "OPENAI_URL" not exists"#);
    let openai_url = Url::parse(&openai_url).expect("openai url error");

    assert!(openai_url.host_str() == Some("api.openai.com"), "invalid target address");
    
    let client_builder = reqwest::Client::builder();
    let client_builder = match env::var("OPENAI_PROXY") {
        Ok(proxy) => client_builder.proxy(reqwest::Proxy::all(proxy).expect("proxy config error")),
        Err(_) => {
            log::warn!("proxy not set, ensure your network...");
            client_builder
        }
    };
    let client = client_builder.build().expect("reqwest client build error");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(MyData {
                pool: pool.clone(),
                client: client.clone(),
                target_url: openai_url.clone(),
            }))
            .configure(config_app)
            // enable logger
            .wrap(middleware::Logger::default())
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

#[cfg(test)]
mod tests {
    use actix_web::{body::to_bytes, dev::Service, http, test, web, App, Error};
    use chatgpt_agent::handlers::index;

    use super::*;

    #[actix_web::test]
    async fn test_index() -> Result<(), Error> {
        let app = App::new().route("/", web::get().to(index::index));
        let app = test::init_service(app).await;

        let req = test::TestRequest::get().uri("/").to_request();
        let resp = app.call(req).await?;

        assert_eq!(resp.status(), http::StatusCode::OK);

        let response_body = resp.into_body();
        assert_eq!(to_bytes(response_body).await?, r##"Hello world!"##);

        Ok(())
    }
}