use redis;

use crate::entity::*;

#[tauri::command]
pub async fn zadd(
    conn: Connection,
    key: String,
    value: Vec<ZsetValue>,
    ttl: i64,
    db: String,
) -> Response<i32> {
    let conn_str = format!(
        "redis://{}:{}@{}:{}/{}",
        "", conn.info.pass, conn.info.host, conn.info.port, db
    );

    let client = redis::Client::open(conn_str).expect("client");
    let mut con: redis::Connection = client.get_connection().expect("con");
    let mut cmd = redis::cmd("ZADD");
    let mut cmd = cmd.arg(&key);
    for v in value {
        cmd = cmd.arg(&v.score).arg(&v.value);
    }
    let result: i32 = cmd.query(&mut con).expect("zadd");
    if ttl > 0 {
        let _res: i32 = redis::cmd("EXPIRE")
            .arg(&key)
            .arg(ttl)
            .query(&mut con)
            .expect("zadd ttl");
    }
    Response::ok(result)
}

#[tauri::command]
pub async fn zrem(conn: Connection, key: String, value: Vec<String>, db: String) -> Response<i32> {
    let conn_str = format!(
        "redis://{}:{}@{}:{}/{}",
        "", conn.info.pass, conn.info.host, conn.info.port, db
    );

    let client = redis::Client::open(conn_str).expect("client");
    let mut con: redis::Connection = client.get_connection().expect("con");
    let mut cmd = redis::cmd("ZREM");
    let mut cmd = cmd.arg(&key);
    for v in value {
        cmd = cmd.arg(&v);
    }
    let result: i32 = cmd.query(&mut con).expect("zrem");
    Response::ok(result)
}
