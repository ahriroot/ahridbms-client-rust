use redis;

use crate::entity::*;

// LPUSH key value [value ...]
#[tauri::command]
pub async fn lpush(
    conn: Connection,
    key: String,
    value: Vec<String>,
    ttl: i64,
    db: String,
) -> Response<i32> {
    let conn_str = format!(
        "redis://{}:{}@{}:{}/{}",
        "", conn.info.pass, conn.info.host, conn.info.port, db
    );

    let client = redis::Client::open(conn_str).expect("client");
    let mut con: redis::Connection = client.get_connection().expect("con");

    let mut cmd = redis::cmd("LPUSH");
    let mut cmd = cmd.arg(&key);
    for v in value {
        cmd = cmd.arg(&v);
    }
    let result: i32 = cmd.query(&mut con).expect("lpush");
    if ttl > 0 {
        let _res: i32 = redis::cmd("EXPIRE")
            .arg(&key)
            .arg(ttl)
            .query(&mut con)
            .expect("lpush ttl");
    }
    Response::ok(result)
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
    let conn_str = format!(
        "redis://{}:{}@{}:{}/{}",
        "", conn.info.pass, conn.info.host, conn.info.port, db
    );

    let client = redis::Client::open(conn_str).expect("client");
    let mut con: redis::Connection = client.get_connection().expect("con");

    let mut cmd = redis::cmd("RPUSH");
    let mut cmd = cmd.arg(&key);
    for v in value {
        cmd = cmd.arg(&v);
    }
    let result: i32 = cmd.query(&mut con).expect("rpush");
    if ttl > 0 {
        let _res: i32 = redis::cmd("EXPIRE")
            .arg(&key)
            .arg(ttl)
            .query(&mut con)
            .expect("rpush ttl");
    } else if ttl == -1 {
        let _res: i32 = redis::cmd("PERSIST")
            .arg(&key)
            .query(&mut con)
            .expect("rpush persist");
    }
    Response::ok(result)
}

// LPOP key
#[tauri::command]
pub async fn lpop(conn: Connection, key: String, db: String) -> Response<String> {
    let conn_str = format!(
        "redis://{}:{}@{}:{}/{}",
        "", conn.info.pass, conn.info.host, conn.info.port, db
    );

    let client = redis::Client::open(conn_str).expect("client");
    let mut con: redis::Connection = client.get_connection().expect("con");

    let result: String = redis::cmd("LPOP").arg(&key).query(&mut con).expect("lpop");
    Response::ok(result)
}

// RPOP key
#[tauri::command]
pub async fn rpop(conn: Connection, key: String, db: String) -> Response<String> {
    let conn_str = format!(
        "redis://{}:{}@{}:{}/{}",
        "", conn.info.pass, conn.info.host, conn.info.port, db
    );

    let client = redis::Client::open(conn_str).expect("client");
    let mut con: redis::Connection = client.get_connection().expect("con");

    let result: String = redis::cmd("RPOP").arg(&key).query(&mut con).expect("rpop");
    Response::ok(result)
}

#[tauri::command]
pub async fn lset(
    conn: Connection,
    key: String,
    index: i64,
    value: String,
    db: String,
) -> Response<String> {
    let conn_str = format!(
        "redis://{}:{}@{}:{}/{}",
        "", conn.info.pass, conn.info.host, conn.info.port, db
    );

    let client = redis::Client::open(conn_str).expect("client");
    let mut con: redis::Connection = client.get_connection().expect("con");

    let result: String = redis::cmd("LSET")
        .arg(&key)
        .arg(index)
        .arg(&value)
        .query(&mut con)
        .expect("lset");
    Response::ok(result)
}
