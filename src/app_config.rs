use actix_web::web;

use crate::handlers::{parts, products, index, chat, user_token};

pub fn config_app(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/index.html").to(|| async { "Hello world!" }));
    cfg.service(web::resource("/").to(index::index));
    cfg.service(web::scope("/data")
            .service(web::resource("/week")
                .route(web::get().to(user_token::weekly_tokens))));
    cfg.service(web::scope("/chat")
            .service(web::resource("")
                .route(web::post().to(chat::chat_to))));
    // domain includes: /products/{product_id}/parts/{part_id}
    cfg.service(
        web::scope("/products")
            .service(
                web::resource("")
                    .route(web::get().to(products::get_products))
                    .route(web::post().to(products::add_product)),
            )
            .service(
                web::scope("/{product_id}")
                    .service(
                        web::resource("")
                            .route(web::get().to(products::get_product_detail))
                            .route(web::delete().to(products::remove_product)),
                    )
                    .service(
                        web::scope("/parts")
                            .service(
                                web::resource("")
                                    .route(web::get().to(parts::get_parts))
                                    .route(web::post().to(parts::add_part)),
                            )
                            .service(
                                web::resource("/{part_id}")
                                    .route(web::get().to(parts::get_part_detail))
                                    .route(web::delete().to(parts::remove_part)),
                            ),
                    ),
            ),
    );
}