use redis;

use crate::entity::*;

#[tauri::command]
pub async fn hset(
    conn: Connection,
    key: String,
    value: Vec<HashValue>,
    ttl: i64,
    db: String,
) -> Response<String> {
    let conn_str = format!(
        "redis://{}:{}@{}:{}/{}",
        "", conn.info.pass, conn.info.host, conn.info.port, db
    );

    let client = redis::Client::open(conn_str).expect("client");
    let mut con: redis::Connection = client.get_connection().expect("con");
    let mut cmd = redis::cmd("HMSET");
    let mut cmd = cmd.arg(&key);
    for v in value {
        cmd = cmd.arg(&v.field).arg(&v.value);
    }
    let result: String = cmd.query(&mut con).expect("hmset");
    if ttl > 0 {
        let _res: i32 = redis::cmd("EXPIRE")
            .arg(&key)
            .arg(ttl)
            .query(&mut con)
            .expect("hmset ttl");
    }
    Response::ok(result)
}

#[tauri::command]
pub async fn hdel(conn: Connection, key: String, fields: Vec<String>, db: String) -> Response<i32> {
    let conn_str = format!(
        "redis://{}:{}@{}:{}/{}",
        "", conn.info.pass, conn.info.host, conn.info.port, db
    );

    let client = redis::Client::open(conn_str).expect("client");
    let mut con: redis::Connection = client.get_connection().expect("con");
    let mut cmd = redis::cmd("HDEL");
    let mut cmd = cmd.arg(&key);
    for field in fields {
        cmd = cmd.arg(&field);
    }
    let result: i32 = cmd.query(&mut con).expect("hdel");
    Response::ok(result)
}
