use redis;

use crate::{entity::*, utils::get_connection};

pub async fn handle_json_set(
    conn: Connection,
    key: String,
    path: String,
    value: String,
    ttl: i64,
    db: String,
) -> redis::RedisResult<String> {
    let mut con = get_connection(conn, db).await?;

    let mut cmd = redis::cmd("JSON.SET");
    let mut cmd = cmd.arg(&key);
    cmd = cmd.arg(&path);
    cmd = cmd.arg(&value);
    let result: String = cmd.query(&mut con)?;
    if ttl > 0 {
        let _res: i32 = redis::cmd("EXPIRE").arg(&key).arg(ttl).query(&mut con)?;
    } else if ttl == -1 {
        let _res: i32 = redis::cmd("PERSIST").arg(&key).query(&mut con)?;
    }
    Ok(result)
}

#[tauri::command]
pub async fn json_set(
    conn: Connection,
    key: String,
    path: String,
    value: String,
    ttl: i64,
    db: String,
) -> Response<String> {
    let result = handle_json_set(conn, key, path, value, ttl, db).await;
    match result {
        Ok(res) => Response::ok(Res::Success(res)),
        Err(e) => Response::error(e.to_string()),
    }
}
