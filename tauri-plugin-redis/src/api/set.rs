use redis;

use crate::{entity::*, utils::get_connection};

pub async fn handle_sadd(
    conn: Connection,
    key: String,
    value: Vec<String>,
    ttl: i64,
    db: String,
) -> redis::RedisResult<i32> {
    let mut con = get_connection(conn, db).await?;

    let mut cmd = redis::cmd("SADD");
    let mut cmd = cmd.arg(&key);
    for v in value {
        cmd = cmd.arg(&v);
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
pub async fn sadd(
    conn: Connection,
    key: String,
    value: Vec<String>,
    ttl: i64,
    db: String,
) -> Response<i32> {
    let result = handle_sadd(conn, key, value, ttl, db).await;
    match result {
        Ok(res) => Response::ok(Res::Success(res)),
        Err(e) => Response::error(e.to_string()),
    }
}

pub async fn handle_srem(
    conn: Connection,
    key: String,
    value: Vec<String>,
    db: String,
) -> redis::RedisResult<i32> {
    let mut con = get_connection(conn, db).await?;

    let mut cmd = redis::cmd("SREM");
    let mut cmd = cmd.arg(&key);
    for v in value {
        cmd = cmd.arg(&v);
    }
    let result: i32 = cmd.query(&mut con)?;
    Ok(result)
}

#[tauri::command]
pub async fn srem(conn: Connection, key: String, value: Vec<String>, db: String) -> Response<i32> {
    let result = handle_srem(conn, key, value, db).await;
    match result {
        Ok(res) => Response::ok(Res::Success(res)),
        Err(e) => Response::error(e.to_string()),
    }
}
