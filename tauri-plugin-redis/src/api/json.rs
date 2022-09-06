use redis;

use crate::entity::*;

#[tauri::command]
pub async fn json_set(
    conn: Connection,
    key: String,
    path: String,
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
    let mut cmd = redis::cmd("JSON.SET");
    let mut cmd = cmd.arg(&key);
    cmd = cmd.arg(&path);
    cmd = cmd.arg(&value);
    let result: String = cmd.query(&mut con).expect("json.set");
    if ttl > 0 {
        let _res: i32 = redis::cmd("EXPIRE")
            .arg(&key)
            .arg(ttl)
            .query(&mut con)
            .expect("json.set ttl");
    } else if ttl == -1 {
        let _res: i32 = redis::cmd("PERSIST")
            .arg(&key)
            .query(&mut con)
            .expect("json.set persist");
    }
    Response::ok(result)
}
