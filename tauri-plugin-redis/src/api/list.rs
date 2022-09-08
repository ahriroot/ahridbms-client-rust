use redis;

use crate::{entity::*, utils::get_connection};

pub async fn handle_lpush(
    conn: Connection,
    key: String,
    value: Vec<String>,
    ttl: i64,
    db: String,
) -> redis::RedisResult<i32> {
    let mut con = get_connection(conn, db).await?;

    let mut cmd = redis::cmd("LPUSH");
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

// LPUSH key value [value ...]
#[tauri::command]
pub async fn lpush(
    conn: Connection,
    key: String,
    value: Vec<String>,
    ttl: i64,
    db: String,
) -> Response<i32> {
    let result = handle_lpush(conn, key, value, ttl, db).await;
    match result {
        Ok(res) => Response::ok(Res::Success(res)),
        Err(e) => Response::error(e.to_string()),
    }
}

pub async fn handle_rpush(
    conn: Connection,
    key: String,
    value: Vec<String>,
    ttl: i64,
    db: String,
) -> redis::RedisResult<i32> {
    let mut con = get_connection(conn, db).await?;

    let mut cmd = redis::cmd("RPUSH");
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

// RPUSH key value [value ...]
#[tauri::command]
pub async fn rpush(
    conn: Connection,
    key: String,
    value: Vec<String>,
    ttl: i64,
    db: String,
) -> Response<i32> {
    let result = handle_rpush(conn, key, value, ttl, db).await;
    match result {
        Ok(res) => Response::ok(Res::Success(res)),
        Err(e) => Response::error(e.to_string()),
    }
}

pub async fn handle_lpop(conn: Connection, key: String, db: String) -> redis::RedisResult<String> {
    let mut con = get_connection(conn, db).await?;

    let result: String = redis::cmd("LPOP").arg(&key).query(&mut con)?;
    Ok(result)
}

// LPOP key
#[tauri::command]
pub async fn lpop(conn: Connection, key: String, db: String) -> Response<String> {
    let result = handle_lpop(conn, key, db).await;
    match result {
        Ok(res) => Response::ok(Res::Success(res)),
        Err(e) => Response::error(e.to_string()),
    }
}

pub async fn handle_rpop(conn: Connection, key: String, db: String) -> redis::RedisResult<String> {
    let mut con = get_connection(conn, db).await?;

    let result: String = redis::cmd("RPOP").arg(&key).query(&mut con)?;
    Ok(result)
}

// RPOP key
#[tauri::command]
pub async fn rpop(conn: Connection, key: String, db: String) -> Response<String> {
    let result = handle_rpop(conn, key, db).await;
    match result {
        Ok(res) => Response::ok(Res::Success(res)),
        Err(e) => Response::error(e.to_string()),
    }
}

pub async fn handle_lset(
    conn: Connection,
    key: String,
    index: i64,
    value: String,
    db: String,
) -> redis::RedisResult<String> {
    let mut con = get_connection(conn, db).await?;

    let result: String = redis::cmd("LSET")
        .arg(&key)
        .arg(index)
        .arg(&value)
        .query(&mut con)?;
    Ok(result)
}

#[tauri::command]
pub async fn lset(
    conn: Connection,
    key: String,
    index: i64,
    value: String,
    db: String,
) -> Response<String> {
    let result = handle_lset(conn, key, index, value, db).await;
    match result {
        Ok(res) => Response::ok(Res::Success(res)),
        Err(e) => Response::error(e.to_string()),
    }
}
