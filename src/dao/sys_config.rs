use sqlx::PgPool;

pub async fn get_config(db: &PgPool, group: String, key: String) -> Option<String> {
    sqlx::query!(r#"
        SELECT value
        from "sys_config" 
        where "group"=$1 and key=$2"#,
        group, key
    )
        .fetch_one(db)
        .await
        .map(|d| d.value)
        .unwrap_or_else(|_| {
            log::error!("query {group}:{key} doesnot exists.");
            None
        })
}