use sqlx::{PgPool, QueryBuilder, Postgres};

use crate::{Usage, GROUP_CHAT_PRICE, CHAT_INPUT_SUFFIX, CHAT_OUTPUT_SUFFIX, SysModelRequest};

use super::sys_config::get_config;

fn caculate_price(usage: Usage, in_price: f32, out_price:f32) -> f32 {
    usage.prompt_tokens as f32 * in_price / 1000.0 + usage.completion_tokens as f32 * out_price / 1000.0
}

pub async fn save_token(db: &PgPool, user_id: i32, remote_id: String, model: String, usage: Usage) {
    let mut query_builder: QueryBuilder<Postgres> = QueryBuilder::new(r#"INSERT INTO "sys_model_request"(token_id, remote_id, model, completion_tokens, prompt_tokens, total_tokens"#);

    let input_price = get_config(db, GROUP_CHAT_PRICE.to_string(), format!("{model}{CHAT_INPUT_SUFFIX}"))
            .await
            .and_then(|price| match price.parse::<f32>() {
                Ok(p) => Some(p),
                Err(_) => None,
            });
    let output_price = get_config(db, GROUP_CHAT_PRICE.to_string(), format!("{model}{CHAT_OUTPUT_SUFFIX}"))
            .await
            .and_then(|price| match price.parse::<f32>() {
                Ok(p) => Some(p),
                Err(_) => None,
            });

    if input_price.is_some() && output_price.is_some() {
        query_builder.push(", define_price, price) values (");
    } else {
        log::warn!("model {model} has no input/output price, please check...");
        query_builder.push(") values (");
    }

    let mut separated = query_builder.separated(", ");

    separated.push_bind(user_id);
    separated.push_bind(remote_id);
    separated.push_bind(model);
    separated.push_bind(usage.completion_tokens);
    separated.push_bind(usage.prompt_tokens);
    separated.push_bind(usage.total_tokens);

    if input_price.is_some() && output_price.is_some() {
        separated.push_bind(format!("{},{}", input_price.unwrap(), output_price.unwrap()));
        separated.push_bind(caculate_price(usage, input_price.unwrap(), output_price.unwrap()));
    }

    separated.push_unseparated(")");


    if let Err(_) = query_builder
        .build()
        .execute(db)
        .await {
        log::error!("sql insert error...");
    }
}

/// 获取 7 天的请求数据
pub async fn get_week_tokens(db: &PgPool, token: String) -> Vec<SysModelRequest> {
    get_days_tokens(db, token, 7).await
}

async fn get_days_tokens(db: &PgPool, token: String, days: i32) -> Vec<SysModelRequest> {
    sqlx::query_as::<_, SysModelRequest>(r#"
        SELECT remote_id, model, total_tokens, price, timestamp
        from "sys_model_request" r
        left join "sys_user_token" t on r.token_id = t.id
        where t.local_token = $1 and r."timestamp" >= now() - make_interval(days := $2)"#
    )
        .bind(token.clone())
        .bind(days)
        .fetch_all(db)
        .await
        .unwrap_or_else(|_| {
            log::error!("database query error.. {token}:{days}");
            Vec::default()
        })
}