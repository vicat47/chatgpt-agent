use std::env;

use actix_web::{middleware, App, HttpServer, web};
use chatgpt_agent::{app_config::config_app, MyData, DEFAULT_CHATGPT_URL};
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

    let db_url=format!("postgres://{}:{}@{}/{}", 
        env::var("DATABASE_USERNAME").unwrap_or_else(|_| {
            log::warn!(r#"will use default username "postgres""#);
            "postgres".to_string()
        }),
        env::var("DATABASE_PASSWORD").unwrap_or_else(|_| {
            log::warn!(r#"will use default postgres password "postgres""#);
            "postgres".to_string()
        }),
        env::var("DATABASE_ADDRESS").unwrap_or_else(|_| {
            log::warn!(r#"will use default postgres address "database:5432""#);
            "database:5432".to_string()
        }),
        env::var("DATABASE_NAME").unwrap_or_else(|_|  {
            log::warn!(r#"will use default postgres database "postgres""#);
            "postgres".to_string()
        })
    );

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url).await.expect("database init failed");

    let openai_url = env::var("OPENAI_URL").unwrap_or_else(|_|  {
        let default = DEFAULT_CHATGPT_URL.to_string();
        log::warn!(r#"will use default open ai api url "{}""#, default);
        default
    });
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

    log::info!("starting HTTP server at http://localhost:8080");

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
    .bind(("0.0.0.0", 8080))?
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