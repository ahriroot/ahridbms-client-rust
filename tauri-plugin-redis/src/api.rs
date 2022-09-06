pub mod hash;
pub mod json;
pub mod list;
pub mod set;
pub mod string;
pub mod zset;

use redis;
use std::collections::{HashMap, HashSet};

use crate::entity::*;

#[tauri::command]
pub async fn keys(conn: Connection, arg: String, db: String) -> Response<Vec<KeyValue>> {
    let conn_str = format!(
        "redis://{}:{}@{}:{}/{}",
        "", conn.info.pass, conn.info.host, conn.info.port, db
    );

    let mut all_data: Vec<KeyValue> = Vec::new();
    let client = redis::Client::open(conn_str).expect("client");
    let mut con: redis::Connection = client.get_connection().expect("con");
    let res: Vec<String> = redis::cmd("keys").arg(&arg).query(&mut con).expect("res");
    // 遍历所有的key
    for key in &res {
        // 获取key的类型
        let key_type: String = redis::cmd("type")
            .arg(&key)
            .query(&mut con)
            .expect("key_type");

        // key_type 以 json 开头
        if key_type == "ReJSON-RL" {
            let value: String = redis::cmd("JSON.GET")
                .arg(&key)
                .query(&mut con)
                .expect("value");
            let size: i64 = redis::cmd("MEMORY")
                .arg("usage")
                .arg(&key)
                .query(&mut con)
                .expect("size");
            let ttl: i64 = redis::cmd("TTL").arg(&key).query(&mut con).expect("ttl");
            let kv = KeyValue::ReJson(KV {
                key: key.to_string(),
                key_type: key_type.to_string(),
                value: value,
                size: size,
                ttl: ttl,
            });
            all_data.push(kv);
        } else if key_type == "string" {
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
        } else if key_type == "set" {
            let value: HashSet<String> = redis::cmd("SMEMBERS")
                .arg(&key)
                .query(&mut con)
                .expect("value");
            let size: i64 = redis::cmd("MEMORY")
                .arg("usage")
                .arg(&key)
                .query(&mut con)
                .expect("size");
            let ttl: i64 = redis::cmd("TTL").arg(&key).query(&mut con).expect("ttl");
            let kv = KeyValue::Set(KV {
                key: key.to_string(),
                key_type: key_type.to_string(),
                value: value,
                size: size,
                ttl: ttl,
            });
            all_data.push(kv);
        } else if key_type == "zset" {
            let value: Vec<String> = redis::cmd("ZRANGE")
                .arg(&key)
                .arg(0)
                .arg(-1)
                .arg("WITHSCORES")
                .query(&mut con)
                .expect("value");
            let size: i64 = redis::cmd("MEMORY")
                .arg("usage")
                .arg(&key)
                .query(&mut con)
                .expect("size");
            let ttl: i64 = redis::cmd("TTL").arg(&key).query(&mut con).expect("ttl");
            let kv = KeyValue::Zset(KV {
                key: key.to_string(),
                key_type: key_type.to_string(),
                value: value,
                size: size,
                ttl: ttl,
            });
            all_data.push(kv);
        } else if key_type == "hash" {
            let value: HashMap<String, String> = redis::cmd("HGETALL")
                .arg(&key)
                .query(&mut con)
                .expect("value");
            let size: i64 = redis::cmd("MEMORY")
                .arg("usage")
                .arg(&key)
                .query(&mut con)
                .expect("size");
            let ttl: i64 = redis::cmd("TTL").arg(&key).query(&mut con).expect("ttl");
            let kv = KeyValue::Hash(KV {
                key: key.to_string(),
                key_type: key_type.to_string(),
                value: value,
                size: size,
                ttl: ttl,
            });
            all_data.push(kv);
        } else if key_type == "none" {
            let kv = KeyValue::Null(KV {
                key: key.to_string(),
                key_type: key_type.to_string(),
                value: "".to_string(),
                size: 0,
                ttl: 0,
            });
            all_data.push(kv);
        }
    }
    Response::ok(all_data)
}

#[tauri::command]
pub async fn get(conn: Connection, key: String, db: String) -> Response<KeyValue> {
    let conn_str = format!(
        "redis://{}:{}@{}:{}/{}",
        "", conn.info.pass, conn.info.host, conn.info.port, db
    );

    let client = redis::Client::open(conn_str).expect("client");
    let mut con: redis::Connection = client.get_connection().expect("con");

    let key_type: String = redis::cmd("type")
        .arg(&key)
        .query(&mut con)
        .expect("key_type");

    let result: Response<KeyValue>;

    if key_type == "ReJSON-RL" {
        let value: String = redis::cmd("JSON.GET")
            .arg(&key)
            .query(&mut con)
            .expect("value");
        let size: i64 = redis::cmd("MEMORY")
            .arg("usage")
            .arg(&key)
            .query(&mut con)
            .expect("size");
        let ttl: i64 = redis::cmd("TTL").arg(&key).query(&mut con).expect("ttl");
        result = Response::ok(KeyValue::ReJson(KV {
            key: key.to_string(),
            key_type: key_type.to_string(),
            value: value,
            size: size,
            ttl: ttl,
        }));
    } else if key_type == "string" {
        let value: String = redis::cmd("GET").arg(&key).query(&mut con).expect("value");
        let size: i64 = redis::cmd("MEMORY")
            .arg("usage")
            .arg(&key)
            .query(&mut con)
            .expect("size");
        let ttl: i64 = redis::cmd("TTL").arg(&key).query(&mut con).expect("ttl");
        result = Response::ok(KeyValue::String(KV {
            key: key.to_string(),
            key_type: key_type.to_string(),
            value: value,
            size: size,
            ttl: ttl,
        }));
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
        result = Response::ok(KeyValue::List(KV {
            key: key.to_string(),
            key_type: key_type.to_string(),
            value: value,
            size: size,
            ttl: ttl,
        }));
    } else if key_type == "set" {
        let value: HashSet<String> = redis::cmd("SMEMBERS")
            .arg(&key)
            .query(&mut con)
            .expect("value");
        let size: i64 = redis::cmd("MEMORY")
            .arg("usage")
            .arg(&key)
            .query(&mut con)
            .expect("size");
        let ttl: i64 = redis::cmd("TTL").arg(&key).query(&mut con).expect("ttl");
        result = Response::ok(KeyValue::Set(KV {
            key: key.to_string(),
            key_type: key_type.to_string(),
            value: value,
            size: size,
            ttl: ttl,
        }));
    } else if key_type == "zset" {
        let value: Vec<String> = redis::cmd("ZRANGE")
            .arg(&key)
            .arg(0)
            .arg(-1)
            .arg("WITHSCORES")
            .query(&mut con)
            .expect("value");
        let size: i64 = redis::cmd("MEMORY")
            .arg("usage")
            .arg(&key)
            .query(&mut con)
            .expect("size");
        let ttl: i64 = redis::cmd("TTL").arg(&key).query(&mut con).expect("ttl");
        result = Response::ok(KeyValue::Zset(KV {
            key: key.to_string(),
            key_type: key_type.to_string(),
            value: value,
            size: size,
            ttl: ttl,
        }));
    } else if key_type == "hash" {
        let value: HashMap<String, String> = redis::cmd("HGETALL")
            .arg(&key)
            .query(&mut con)
            .expect("value");
        let size: i64 = redis::cmd("MEMORY")
            .arg("usage")
            .arg(&key)
            .query(&mut con)
            .expect("size");
        let ttl: i64 = redis::cmd("TTL").arg(&key).query(&mut con).expect("ttl");
        result = Response::ok(KeyValue::Hash(KV {
            key: key.to_string(),
            key_type: key_type.to_string(),
            value: value,
            size: size,
            ttl: ttl,
        }));
    } else {
        result = Response::ok(KeyValue::Null(KV {
            key: key.to_string(),
            key_type: key_type.to_string(),
            value: "".to_string(),
            size: 0,
            ttl: 0,
        }));
    }
    result
}

#[tauri::command]
pub async fn del(conn: Connection, key: String, db: String) -> Response<String> {
    let conn_str = format!(
        "redis://{}:{}@{}:{}/{}",
        "", conn.info.pass, conn.info.host, conn.info.port, db
    );

    let client = redis::Client::open(conn_str).expect("client");
    let mut con: redis::Connection = client.get_connection().expect("con");

    let _res: i32 = redis::cmd("DEL").arg(&key).query(&mut con).expect("del");
    Response::ok("OK".to_string())
}

#[tauri::command]
pub async fn expire(conn: Connection, key: String, ttl: i64, db: String) -> Response<String> {
    let conn_str = format!(
        "redis://{}:{}@{}:{}/{}",
        "", conn.info.pass, conn.info.host, conn.info.port, db
    );

    let client = redis::Client::open(conn_str).expect("client");
    let mut con: redis::Connection = client.get_connection().expect("con");

    if ttl > 0 {
        let _res: i32 = redis::cmd("EXPIRE")
            .arg(&key)
            .arg(ttl)
            .query(&mut con)
            .expect("del");
    } else if ttl == -1 {
        let _res: i32 = redis::cmd("PERSIST")
            .arg(&key)
            .query(&mut con)
            .expect("persist");
    }
    Response::ok("OK".to_string())
}

#[tauri::command]
pub async fn info(conn: Connection, db: String) -> Response<String> {
    let conn_str = format!(
        "redis://{}:{}@{}:{}/{}",
        "", conn.info.pass, conn.info.host, conn.info.port, db
    );

    let client = redis::Client::open(conn_str).expect("client");
    let mut con: redis::Connection = client.get_connection().expect("con");

    let info: String = redis::cmd("INFO").query(&mut con).expect("keyspace");

    Response::ok(info)
}
