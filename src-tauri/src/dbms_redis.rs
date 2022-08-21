use redis;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct KV<T> {
    key: String,
    key_type: String,
    value: T,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum KeyValue {
    Int(KV<i32>),
    String(KV<String>),
    Float(KV<f64>),
}

#[derive(Debug, Serialize, Deserialize)]
struct Info {
    name: String,
    host: String,
    port: String,
    user: String,
    pass: String,
    index: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Conn {
    id: String,
    db_type: String,
    info: Info,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Connection {
    conn: Conn,
    db: String,
}

#[tauri::command]
pub async fn keys(conn: Connection) -> Vec<KeyValue> {
    let conn_str = format!(
        "redis://{}:{}@{}:{}/{}",
        "", conn.conn.info.pass, conn.conn.info.host, conn.conn.info.port, conn.db
    );

    let mut all_data: Vec<KeyValue> = Vec::new();
    let client = redis::Client::open(conn_str).expect("client");
    let mut con: redis::Connection = client.get_connection().expect("con");
    let res: Vec<String> = redis::cmd("keys").arg("*").query(&mut con).expect("res");
    // 遍历所有的key
    for key in &res {
        // 获取key的类型
        let key_type: String = redis::cmd("type")
            .arg(&key)
            .query(&mut con)
            .expect("key_type");

        if key_type == "string" {
            let value: String = redis::cmd("get").arg(&key).query(&mut con).expect("value");
            let kv = KeyValue::String(KV {
                key: key.to_string(),
                key_type: key_type.to_string(),
                value: value,
            });
            all_data.push(kv);
        }
    }
    all_data
}

#[tauri::command]
pub async fn get(key: String) -> Option<KeyValue> {
    let client = redis::Client::open("redis://:Aa12345.@db.ahriknow.com:6379/0").expect("client");
    let mut con: redis::Connection = client.get_connection().expect("con");

    let key_type: String = redis::cmd("type")
        .arg(&key)
        .query(&mut con)
        .expect("key_type");

    let mut kv: Option<KeyValue> = None;

    if key_type == "string" {
        let value: String = redis::cmd("get").arg(&key).query(&mut con).expect("value");
        kv = Some(KeyValue::String(KV {
            key: key,
            key_type: key_type.to_string(),
            value: value,
        }));
    }
    kv
}

#[tauri::command]
pub async fn key_space() -> String {
    let client = redis::Client::open("redis://:Aa12345.@db.ahriknow.com:6379/0").expect("client");
    let mut con: redis::Connection = client.get_connection().expect("con");

    let key_space_info: String = redis::cmd("INFO").query(&mut con).expect("keyspace");

    key_space_info
}
