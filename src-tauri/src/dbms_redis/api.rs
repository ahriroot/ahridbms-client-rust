use redis;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeSet, HashMap, HashSet};

use crate::dbms_redis::entity::*;

#[tauri::command]
pub async fn keys(conn: Connection, arg: String) -> Response<Vec<KeyValue>> {
    let conn_str = format!(
        "redis://{}:{}@{}:{}/{}",
        "", conn.conn.info.pass, conn.conn.info.host, conn.conn.info.port, conn.db
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
        println!("{}", key_type);

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
            let value: BTreeSet<String> = redis::cmd("ZRANGE")
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
pub async fn get(conn: Connection, key: String) -> Response<KeyValue> {
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

    let result: Response<KeyValue>;

    if key_type == "string" {
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
        let value: BTreeSet<String> = redis::cmd("ZRANGE")
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
pub async fn del(conn: Connection, key: String) -> Response<String> {
    let conn_str = format!(
        "redis://{}:{}@{}:{}/{}",
        "", conn.conn.info.pass, conn.conn.info.host, conn.conn.info.port, conn.db
    );

    let client = redis::Client::open(conn_str).expect("client");
    let mut con: redis::Connection = client.get_connection().expect("con");

    let _res: i32 = redis::cmd("DEL").arg(&key).query(&mut con).expect("del");
    Response::ok("OK".to_string())
}

#[tauri::command]
pub async fn expire(conn: Connection, key: String, ttl: i64) -> Response<String> {
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
    Response::ok("OK".to_string())
}

#[tauri::command]
pub async fn key_space(conn: Connection) -> Response<String> {
    let conn_str = format!(
        "redis://{}:{}@{}:{}/{}",
        "", conn.conn.info.pass, conn.conn.info.host, conn.conn.info.port, conn.db
    );

    let client = redis::Client::open(conn_str).expect("client");
    let mut con: redis::Connection = client.get_connection().expect("con");

    let key_space_info: String = redis::cmd("INFO").query(&mut con).expect("keyspace");

    Response::ok(key_space_info)
}

#[tauri::command]
pub async fn set_string(
    conn: Connection,
    key: String,
    value: String,
    ttl: i64,
) -> Response<String> {
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
        return Response::ok(key_space_info);
    } else {
        let key_space_info: String = redis::cmd("SET")
            .arg(&key)
            .arg(&value)
            .query(&mut con)
            .expect("set_string");
        return Response::ok(key_space_info);
    }
}

#[tauri::command]
pub async fn rpush(conn: Connection, key: String, value: Vec<String>, ttl: i64) -> Response<i32> {
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
    Response::ok(result)
}

#[tauri::command]
pub async fn sadd(conn: Connection, key: String, value: Vec<String>, ttl: i64) -> Response<i32> {
    let conn_str = format!(
        "redis://{}:{}@{}:{}/{}",
        "", conn.conn.info.pass, conn.conn.info.host, conn.conn.info.port, conn.db
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
    }
    Response::ok(result)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ZsetValue {
    score: f64,
    value: String,
}

#[tauri::command]
pub async fn zadd(conn: Connection, key: String, value: Vec<ZsetValue>, ttl: i64) -> Response<i32> {
    println!("{}", key);
    let conn_str = format!(
        "redis://{}:{}@{}:{}/{}",
        "", conn.conn.info.pass, conn.conn.info.host, conn.conn.info.port, conn.db
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
pub async fn srem(conn: Connection, key: String, value: Vec<String>) -> Response<i32> {
    let conn_str = format!(
        "redis://{}:{}@{}:{}/{}",
        "", conn.conn.info.pass, conn.conn.info.host, conn.conn.info.port, conn.db
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

#[derive(Debug, Serialize, Deserialize)]
pub struct HashValue {
    key: String,
    value: String,
}

#[tauri::command]
pub async fn hmset(
    conn: Connection,
    key: String,
    value: Vec<HashValue>,
    ttl: i64,
) -> Response<String> {
    let conn_str = format!(
        "redis://{}:{}@{}:{}/{}",
        "", conn.conn.info.pass, conn.conn.info.host, conn.conn.info.port, conn.db
    );

    let client = redis::Client::open(conn_str).expect("client");
    let mut con: redis::Connection = client.get_connection().expect("con");
    let mut cmd = redis::cmd("HMSET");
    let mut cmd = cmd.arg(&key);
    for v in value {
        cmd = cmd.arg(&v.key).arg(&v.value);
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
