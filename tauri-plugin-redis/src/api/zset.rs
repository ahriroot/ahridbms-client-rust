use redis;

use crate::{entity::*, utils::get_connection};

pub async fn handle_zadd(
    conn: Connection,
    key: String,
    value: Vec<ZsetValue>,
    ttl: i64,
    db: String,
) -> redis::RedisResult<i32> {
    let mut con = get_connection(conn, db).await?;

    let mut cmd = redis::cmd("ZADD");
    let mut cmd = cmd.arg(&key);
    for v in value {
        cmd = cmd.arg(&v.score).arg(&v.value);
    }
    let result: i32 = cmd.query(&mut con)?;
    if ttl > 0 {
        let _res: i32 = redis::cmd("EXPIRE")
            .arg(&key)
            .arg(ttl)
            .query(&mut con)?;
    }
    Ok(result)
}

#[tauri::command]
pub async fn zadd(
    conn: Connection,
    key: String,
    value: Vec<ZsetValue>,
    ttl: i64,
    db: String,
) -> Response<i32> {
    let result = handle_zadd(conn, key, value, ttl, db).await;
    match result {
        Ok(res) => Response::ok(Res::Success(res)),
        Err(e) => Response::error(e.to_string()),
    }
}

pub async fn handle_zrem(
    conn: Connection,
    key: String,
    value: Vec<String>,
    db: String,
) -> redis::RedisResult<i32> {
    let mut con = get_connection(conn, db).await?;

    let mut cmd = redis::cmd("ZREM");
    let mut cmd = cmd.arg(&key);
    for v in value {
        cmd = cmd.arg(&v);
    }
    let result: i32 = cmd.query(&mut con)?;
    Ok(result)
}

#[tauri::command]
pub async fn zrem(conn: Connection, key: String, value: Vec<String>, db: String) -> Response<i32> {
    let result = handle_zrem(conn, key, value, db).await;
    match result {
        Ok(res) => Response::ok(Res::Success(res)),
        Err(e) => Response::error(e.to_string()),
    }
}
