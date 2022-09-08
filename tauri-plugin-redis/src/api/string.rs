use redis;

use crate::{entity::*, utils::get_connection};

pub async fn handle_set(
    conn: Connection,
    key: String,
    value: String,
    ttl: i64,
    db: String,
) -> redis::RedisResult<String> {
    let mut con = get_connection(conn, db).await?;

    if ttl > 0 {
        let key_space_info: String = redis::cmd("SETEX")
            .arg(&key)
            .arg(ttl)
            .arg(&value)
            .query(&mut con)?;
        return Ok(key_space_info);
    } else {
        let key_space_info: String = redis::cmd("SET").arg(&key).arg(&value).query(&mut con)?;
        return Ok(key_space_info);
    }
}

#[tauri::command]
pub async fn set(
    conn: Connection,
    key: String,
    value: String,
    ttl: i64,
    db: String,
) -> Response<String> {
    let result = handle_set(conn, key, value, ttl, db).await;
    match result {
        Ok(res) => Response::ok(Res::Success(res)),
        Err(e) => Response::error(e.to_string()),
    }
}

pub async fn handle_reset(
    conn: Connection,
    key: String,
    value: String,
    ttl: i64,
    db: String,
) -> redis::RedisResult<String> {
    let mut con = get_connection(conn, db).await?;

    if ttl > 0 {
        let key_space_info: String = redis::cmd("SETEX")
            .arg(&key)
            .arg(ttl)
            .arg(&value)
            .query(&mut con)?;
        return Ok(key_space_info);
    } else {
        let key_space_info: String = redis::cmd("SET").arg(&key).arg(&value).query(&mut con)?;
        return Ok(key_space_info);
    }
}

#[tauri::command]
pub async fn reset(conn: Connection, key: String, value: String, db: String) -> Response<String> {
    let result = handle_reset(conn, key, value, 0, db).await;
    match result {
        Ok(res) => Response::ok(Res::Success(res)),
        Err(e) => Response::error(e.to_string()),
    }
}
