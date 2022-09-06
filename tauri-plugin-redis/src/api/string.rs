use redis;

use crate::entity::*;

#[tauri::command]
pub async fn set(
    conn: Connection,
    key: String,
    value: String,
    ttl: i64,
    db: String,
) -> Response<String> {
    let conn_str = format!(
        "redis://{}:{}@{}:{}/{}",
        "", conn.info.pass, conn.info.host, conn.info.port, db
    );

    let client = redis::Client::open(conn_str).expect("client");
    let mut con: redis::Connection = client.get_connection().expect("con");

    if ttl > 0 {
        let key_space_info: String = redis::cmd("SETEX")
            .arg(&key)
            .arg(ttl)
            .arg(&value)
            .query(&mut con)
            .expect("set");
        return Response::ok(key_space_info);
    } else {
        let key_space_info: String = redis::cmd("SET")
            .arg(&key)
            .arg(&value)
            .query(&mut con)
            .expect("set");
        return Response::ok(key_space_info);
    }
}

#[tauri::command]
pub async fn reset(conn: Connection, key: String, value: String, db: String) -> Response<String> {
    let conn_str = format!(
        "redis://{}:{}@{}:{}/{}",
        "", conn.info.pass, conn.info.host, conn.info.port, db
    );

    let client = redis::Client::open(conn_str).expect("client");
    let mut con: redis::Connection = client.get_connection().expect("con");

    let ttl: i64 = redis::cmd("TTL").arg(&key).query(&mut con).expect("ttl");

    if ttl > 0 {
        let key_space_info: String = redis::cmd("SETEX")
            .arg(&key)
            .arg(ttl)
            .arg(&value)
            .query(&mut con)
            .expect("reset");
        return Response::ok(key_space_info);
    } else {
        let key_space_info: String = redis::cmd("SET")
            .arg(&key)
            .arg(&value)
            .query(&mut con)
            .expect("reset");
        return Response::ok(key_space_info);
    }
}
