use redis;

use crate::{entity::*, utils::get_connection};

pub async fn handle_hset(
    conn: Connection,
    key: String,
    value: Vec<HashValue>,
    ttl: i64,
    db: String,
) -> redis::RedisResult<String> {
    let mut con = get_connection(conn, db).await?;

    let mut cmd = redis::cmd("HMSET");
    let mut cmd = cmd.arg(&key);
    for v in value {
        cmd = cmd.arg(&v.field).arg(&v.value);
    }
    let result: String = cmd.query(&mut con)?;
    if ttl > 0 {
        let _res: i32 = redis::cmd("EXPIRE").arg(&key).arg(ttl).query(&mut con)?;
    }
    Ok(result)
}

#[tauri::command]
pub async fn hset(
    conn: Connection,
    key: String,
    value: Vec<HashValue>,
    ttl: i64,
    db: String,
) -> Response<String> {
    let result = handle_hset(conn, key, value, ttl, db).await;
    match result {
        Ok(res) => Response::ok(Res::Success(res)),
        Err(e) => Response::error(e.to_string()),
    }
}

pub async fn handle_hdel(
    conn: Connection,
    key: String,
    fields: Vec<String>,
    db: String,
) -> redis::RedisResult<i32> {
    let mut con = get_connection(conn, db).await?;

    let mut cmd = redis::cmd("HDEL");
    let mut cmd = cmd.arg(&key);
    for field in fields {
        cmd = cmd.arg(&field);
    }
    let result: i32 = cmd.query(&mut con)?;
    Ok(result)
}

#[tauri::command]
pub async fn hdel(conn: Connection, key: String, fields: Vec<String>, db: String) -> Response<i32> {
    let result = handle_hdel(conn, key, fields, db).await;
    match result {
        Ok(res) => Response::ok(Res::Success(res)),
        Err(e) => Response::error(e.to_string()),
    }
}
