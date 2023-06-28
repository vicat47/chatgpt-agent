use std::env;

use actix_web::{middleware, App, HttpServer, web};
use chatgpt_agent::app_config::config_app;
use sqlx::{postgres::PgPoolOptions, types::chrono::{DateTime, Local}};
use dotenv::dotenv;

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
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    log::info!("starting HTTP server at http://localhost:8080");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&env::var("DATABASE_URL").unwrap()).await.unwrap();

    let shared_data = web::Data::new(pool);

    HttpServer::new(move || {
        App::new()
            .app_data(shared_data.clone())
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