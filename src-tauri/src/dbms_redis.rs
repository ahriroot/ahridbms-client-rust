use redis;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct KV<T> {
    key: String,
    key_type: String,
    value: T,
    size: i64,
    ttl: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum KeyValue {
    Int(KV<i32>),
    String(KV<String>),
    Float(KV<f64>),
    List(KV<Vec<String>>),
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
            let size: i64 = redis::cmd("MEMORY")
                .arg("usage")
                .arg(&key)
                .query(&mut con)
                .expect("size");
            let ttl: i64 = redis::cmd("TTL").arg(&key).query(&mut con).expect("ttl");
            let kv = KeyValue::String(KV {
                key: key.to_string(),
                key_type: key_type.to_string(),
                value: value,
                size: size,
                ttl: ttl,
            });
            all_data.push(kv);
        } else if key_type == "list" {
            let value: Vec<String> = redis::cmd("LRANGE")
                .arg(&key)
                .arg(0)
                .arg(-1)
                .query(&mut con)
                .expect("value");
            let size: i64 = redis::cmd("MEMORY")
                .arg("usage")
                .arg(&key)
                .query(&mut con)
                .expect("size");
            let ttl: i64 = redis::cmd("TTL").arg(&key).query(&mut con).expect("ttl");
            let kv = KeyValue::List(KV {
                key: key.to_string(),
                key_type: key_type.to_string(),
                value: value,
                size: size,
                ttl: ttl,
            });
            all_data.push(kv);
        }
        // } else if key_type == "set" {
        //     let value: Vec<String> = redis::cmd("SMEMBERS")
        //         .arg(&key)
        //         .query(&mut con)
        //         .expect("value");
        //     let size: i64 = redis::cmd("MEMORY")
        //         .arg("usage")
        //         .arg(&key)
        //         .query(&mut con)
        //         .expect("size");
        //     let ttl: i64 = redis::cmd("TTL").arg(&key).query(&mut con).expect("ttl");
        //     let kv = KeyValue::String(KV {
        //         key: key.to_string(),
        //         key_type: key_type.to_string(),
        //         value: value,
        //         size: size,
        //         ttl: ttl,
        //     });
        //     all_data.push(kv);
        // } else if key_type == "zset" {
        //     let value: Vec<String> = redis::cmd("ZRANGE")
        //         .arg(&key)
        //         .arg(0)
        //         .arg(-1)
        //         .query(&mut con)
        //         .expect("value");
        //     let size: i64 = redis::cmd("MEMORY")
        //         .arg("usage")
        //         .arg(&key)
        //         .query(&mut con)
        //         .expect("size");
        //     let ttl: i64 = redis::cmd("TTL").arg(&key).query(&mut con).expect("ttl");
        //     let kv = KeyValue::String(KV {
        //         key: key.to_string(),
        //         key_type: key_type.to_string(),
        //         value: value,
        //         size: size,
        //         ttl: ttl,
        //     });
        //     all_data.push(kv);
        // } else if key_type == "hash" {
        //     let value: Vec<String> = redis::cmd("HKEYS")
        //         .arg(&key)
        //         .query(&mut con)
        //         .expect("value");
        //     let size: i64 = redis::cmd("MEMORY")
        //         .arg("usage")
        //         .arg(&key)
        //         .query(&mut con)
        //         .expect("size");
        //     let ttl: i64 = redis::cmd("TTL").arg(&key).query(&mut con).expect("ttl");
        //     let kv = KeyValue::String(KV {
        //         key: key.to_string(),
        //         key_type: key_type.to_string(),
        //         value: value,
        //         size: size,
        //         ttl: ttl,
        //     });
        //     all_data.push(kv);
        // }
    }
    all_data
}

#[tauri::command]
pub async fn get(conn: Connection, key: String) -> Option<KeyValue> {
    let conn_str = format!(
        "redis://{}:{}@{}:{}/{}",
        "", conn.conn.info.pass, conn.conn.info.host, conn.conn.info.port, conn.db
    );

    let client = redis::Client::open(conn_str).expect("client");
    let mut con: redis::Connection = client.get_connection().expect("con");

    let key_type: String = redis::cmd("type")
        .arg(&key)
        .query(&mut con)
        .expect("key_type");

    let mut kv: Option<KeyValue> = None;

    if key_type == "string" {
        let value: String = redis::cmd("get").arg(&key).query(&mut con).expect("value");
        let size: i64 = redis::cmd("MEMORY")
            .arg("usage")
            .arg(&key)
            .query(&mut con)
            .expect("size");
        let ttl: i64 = redis::cmd("TTL").arg(&key).query(&mut con).expect("ttl");
        kv = Some(KeyValue::String(KV {
            key: key,
            key_type: key_type.to_string(),
            value: value,
            size: size,
            ttl: ttl,
        }));
    }
    kv
}

#[tauri::command]
pub async fn del(conn: Connection, key: String) -> String {
    let conn_str = format!(
        "redis://{}:{}@{}:{}/{}",
        "", conn.conn.info.pass, conn.conn.info.host, conn.conn.info.port, conn.db
    );

    let client = redis::Client::open(conn_str).expect("client");
    let mut con: redis::Connection = client.get_connection().expect("con");

    let _res: i32 = redis::cmd("DEL").arg(&key).query(&mut con).expect("del");
    "Ok".to_string()
}

#[tauri::command]
pub async fn expire(conn: Connection, key: String, ttl: i64) -> String {
    let conn_str = format!(
        "redis://{}:{}@{}:{}/{}",
        "", conn.conn.info.pass, conn.conn.info.host, conn.conn.info.port, conn.db
    );

    let client = redis::Client::open(conn_str).expect("client");
    let mut con: redis::Connection = client.get_connection().expect("con");

    let _res: i32 = redis::cmd("EXPIRE")
        .arg(&key)
        .arg(ttl)
        .query(&mut con)
        .expect("del");
    "Ok".to_string()
}

#[tauri::command]
pub async fn key_space(conn: Connection) -> String {
    let conn_str = format!(
        "redis://{}:{}@{}:{}/{}",
        "", conn.conn.info.pass, conn.conn.info.host, conn.conn.info.port, conn.db
    );

    let client = redis::Client::open(conn_str).expect("client");
    let mut con: redis::Connection = client.get_connection().expect("con");

    let key_space_info: String = redis::cmd("INFO").query(&mut con).expect("keyspace");

    key_space_info
}

#[tauri::command]
pub async fn set_string(conn: Connection, key: String, value: String, ttl: i64) -> String {
    let conn_str = format!(
        "redis://{}:{}@{}:{}/{}",
        "", conn.conn.info.pass, conn.conn.info.host, conn.conn.info.port, conn.db
    );

    let client = redis::Client::open(conn_str).expect("client");
    let mut con: redis::Connection = client.get_connection().expect("con");

    if ttl > 0 {
        let key_space_info: String = redis::cmd("SETEX")
            .arg(&key)
            .arg(ttl)
            .arg(&value)
            .query(&mut con)
            .expect("set_string");
        return key_space_info;
    } else {
        let key_space_info: String = redis::cmd("SET")
            .arg(&key)
            .arg(&value)
            .query(&mut con)
            .expect("set_string");
        return key_space_info;
    }
}

#[tauri::command]
pub async fn rpush(conn: Connection, key: String, value: Vec<String>, ttl: i64) -> i32 {
    let conn_str = format!(
        "redis://{}:{}@{}:{}/{}",
        "", conn.conn.info.pass, conn.conn.info.host, conn.conn.info.port, conn.db
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
    }
    result
}
