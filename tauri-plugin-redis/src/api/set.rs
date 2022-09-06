use redis;

use crate::entity::*;

#[tauri::command]
pub async fn sadd(
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

    let mut cmd = redis::cmd("SADD");
    let mut cmd = cmd.arg(&key);
    for v in value {
        cmd = cmd.arg(&v);
    }
    let result: i32 = cmd.query(&mut con).expect("sadd");
    if ttl > 0 {
        let _res: i32 = redis::cmd("EXPIRE")
            .arg(&key)
            .arg(ttl)
            .query(&mut con)
            .expect("sadd ttl");
    } else if ttl == -1 {
        let _res: i32 = redis::cmd("PERSIST")
            .arg(&key)
            .query(&mut con)
            .expect("sadd persist");
    }
    Response::ok(result)
}

#[tauri::command]
pub async fn srem(conn: Connection, key: String, value: Vec<String>, db: String) -> Response<i32> {
    let conn_str = format!(
        "redis://{}:{}@{}:{}/{}",
        "", conn.info.pass, conn.info.host, conn.info.port, db
    );

    let client = redis::Client::open(conn_str).expect("client");
    let mut con: redis::Connection = client.get_connection().expect("con");
    let mut cmd = redis::cmd("SREM");
    let mut cmd = cmd.arg(&key);
    for v in value {
        cmd = cmd.arg(&v);
    }
    let result: i32 = cmd.query(&mut con).expect("srem");
    Response::ok(result)
}
