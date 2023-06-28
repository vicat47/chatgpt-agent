use sqlx::PgPool;

pub async fn get_config(db: &PgPool, group: String, key: String) -> Option<String> {
    match sqlx::query!(r#"
        SELECT value
        from "sys_config" 
        where "group"=$1 and key=$2"#,
        group, key
    )
        .fetch_one(db)
        .await {
            Ok(data) => data.value,
            Err(_) => {
                log::error!("query {group}:{key} doesnot exists.");
                None
            },
        }
}