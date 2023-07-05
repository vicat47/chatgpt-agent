use sqlx::PgPool;

pub async fn get_config(db: &PgPool, group: String, key: String) -> Option<String> {
    sqlx::query!(
        r#"
        SELECT value
        from "sys_config" 
        where "group"=$1 and key=$2"#,
        group,
        key
    )
    .fetch_one(db)
    .await
    .map(|d| d.value)
    .unwrap_or_else(|_| {
        log::error!("query {group}:{key} doesnot exists.");
        None
    })
}

pub async fn check_model_config(db: &PgPool, model: String) -> i64 {
    sqlx::query_as::<_, (i64,)>(
        r#"
        SELECT count(1)
        from "sys_config" 
        where "group" = 'model_price' and "value" IS NOT NULL and ("key" = $1 or "key" = $2 )"#,
    )
    .bind(format!("{}_input", model))
    .bind(format!("{}_output", model))
    .fetch_one(db)
    .await
    .unwrap_or_else(|_| {
        log::error!("query {model} doesnot exists.");
        (0,)
    })
    .0
}
