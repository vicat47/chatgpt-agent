use sqlx::PgPool;

use crate::SysUserToken;

pub async fn get_by_local_token(db: &PgPool, local_token: String) -> Option<SysUserToken> {
    match sqlx::query_as::<_, SysUserToken>(r#"
        SELECT id, name, local_token, gpt_token, create_time 
        from "sys_user_token" 
        where local_token=$1"#)
        .bind(local_token)
        .fetch_one(db)
        .await {
        Ok(data) => {
            Some(data)
        },
        Err(_) => None,
    }
}
