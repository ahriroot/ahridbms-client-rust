pub mod hash;
pub mod json;
pub mod list;
pub mod set;
pub mod string;
pub mod zset;

use redis::{self, ConnectionLike, Value};
use std::collections::{HashMap, HashSet};

use crate::entity::*;
use crate::utils;

pub async fn handle_keys(
    conn: Connection,
    arg: String,
    db: String,
) -> redis::RedisResult<Vec<KeyValue>> {
    let mut con = utils::get_connection(conn, db).await?;
    let mut all_data: Vec<KeyValue> = Vec::new();

    // return Response::err4("".to_string());
    let res: Vec<String> = redis::cmd("keys").arg(&arg).query(&mut con)?;
    // 遍历所有的key
    for key in &res {
        // 获取key的类型
        let key_type: String = redis::cmd("type").arg(&key).query(&mut con)?;

        // key_type 以 json 开头
        if key_type == "ReJSON-RL" {
            let value: String = redis::cmd("JSON.GET").arg(&key).query(&mut con)?;
            let size: i64 = redis::cmd("MEMORY")
                .arg("usage")
                .arg(&key)
                .query(&mut con)?;
            let ttl: i64 = redis::cmd("TTL").arg(&key).query(&mut con)?;
            let kv = KeyValue::ReJson(KV {
                key: key.to_string(),
                key_type: key_type.to_string(),
                value: value,
                size: size,
                ttl: ttl,
            });
            all_data.push(kv);
        } else if key_type == "string" {
            let value: String = redis::cmd("get").arg(&key).query(&mut con)?;
            let size: i64 = redis::cmd("MEMORY")
                .arg("usage")
                .arg(&key)
                .query(&mut con)?;
            let ttl: i64 = redis::cmd("TTL").arg(&key).query(&mut con)?;
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
                .query(&mut con)?;
            let size: i64 = redis::cmd("MEMORY")
                .arg("usage")
                .arg(&key)
                .query(&mut con)?;
            let ttl: i64 = redis::cmd("TTL").arg(&key).query(&mut con)?;
            let kv = KeyValue::List(KV {
                key: key.to_string(),
                key_type: key_type.to_string(),
                value: value,
                size: size,
                ttl: ttl,
            });
            all_data.push(kv);
        } else if key_type == "set" {
            let value: HashSet<String> = redis::cmd("SMEMBERS").arg(&key).query(&mut con)?;
            let size: i64 = redis::cmd("MEMORY")
                .arg("usage")
                .arg(&key)
                .query(&mut con)?;
            let ttl: i64 = redis::cmd("TTL").arg(&key).query(&mut con)?;
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
                .query(&mut con)?;
            let size: i64 = redis::cmd("MEMORY")
                .arg("usage")
                .arg(&key)
                .query(&mut con)?;
            let ttl: i64 = redis::cmd("TTL").arg(&key).query(&mut con)?;
            let kv = KeyValue::Zset(KV {
                key: key.to_string(),
                key_type: key_type.to_string(),
                value: value,
                size: size,
                ttl: ttl,
            });
            all_data.push(kv);
        } else if key_type == "hash" {
            let value: HashMap<String, String> = redis::cmd("HGETALL").arg(&key).query(&mut con)?;
            let size: i64 = redis::cmd("MEMORY")
                .arg("usage")
                .arg(&key)
                .query(&mut con)?;
            let ttl: i64 = redis::cmd("TTL").arg(&key).query(&mut con)?;
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
    Ok(all_data)
}

#[tauri::command]
pub async fn keys(conn: Connection, arg: String, db: String) -> Response<Vec<KeyValue>> {
    let res = handle_keys(conn, arg, db).await;
    match res {
        Ok(v) => Response::ok(Res::Success(v)),
        Err(e) => Response::error(e.to_string()),
    }
}

pub async fn handle_get(conn: Connection, key: String, db: String) -> redis::RedisResult<KeyValue> {
    let mut con: redis::Connection = utils::get_connection(conn, db).await?;

    let key_type: String = redis::cmd("type").arg(&key).query(&mut con)?;

    let result: KeyValue;

    if key_type == "ReJSON-RL" {
        let value: String = redis::cmd("JSON.GET").arg(&key).query(&mut con)?;
        let size: i64 = redis::cmd("MEMORY")
            .arg("usage")
            .arg(&key)
            .query(&mut con)?;
        let ttl: i64 = redis::cmd("TTL").arg(&key).query(&mut con)?;
        result = KeyValue::ReJson(KV {
            key: key.to_string(),
            key_type: key_type.to_string(),
            value: value,
            size: size,
            ttl: ttl,
        });
    } else if key_type == "string" {
        let value: String = redis::cmd("GET").arg(&key).query(&mut con)?;
        let size: i64 = redis::cmd("MEMORY")
            .arg("usage")
            .arg(&key)
            .query(&mut con)?;
        let ttl: i64 = redis::cmd("TTL").arg(&key).query(&mut con)?;
        result = KeyValue::String(KV {
            key: key.to_string(),
            key_type: key_type.to_string(),
            value: value,
            size: size,
            ttl: ttl,
        });
    } else if key_type == "list" {
        let value: Vec<String> = redis::cmd("LRANGE")
            .arg(&key)
            .arg(0)
            .arg(-1)
            .query(&mut con)?;
        let size: i64 = redis::cmd("MEMORY")
            .arg("usage")
            .arg(&key)
            .query(&mut con)?;
        let ttl: i64 = redis::cmd("TTL").arg(&key).query(&mut con)?;
        result = KeyValue::List(KV {
            key: key.to_string(),
            key_type: key_type.to_string(),
            value: value,
            size: size,
            ttl: ttl,
        });
    } else if key_type == "set" {
        let value: HashSet<String> = redis::cmd("SMEMBERS").arg(&key).query(&mut con)?;
        let size: i64 = redis::cmd("MEMORY")
            .arg("usage")
            .arg(&key)
            .query(&mut con)?;
        let ttl: i64 = redis::cmd("TTL").arg(&key).query(&mut con)?;
        result = KeyValue::Set(KV {
            key: key.to_string(),
            key_type: key_type.to_string(),
            value: value,
            size: size,
            ttl: ttl,
        });
    } else if key_type == "zset" {
        let value: Vec<String> = redis::cmd("ZRANGE")
            .arg(&key)
            .arg(0)
            .arg(-1)
            .arg("WITHSCORES")
            .query(&mut con)?;
        let size: i64 = redis::cmd("MEMORY")
            .arg("usage")
            .arg(&key)
            .query(&mut con)?;
        let ttl: i64 = redis::cmd("TTL").arg(&key).query(&mut con)?;
        result = KeyValue::Zset(KV {
            key: key.to_string(),
            key_type: key_type.to_string(),
            value: value,
            size: size,
            ttl: ttl,
        });
    } else if key_type == "hash" {
        let value: HashMap<String, String> = redis::cmd("HGETALL").arg(&key).query(&mut con)?;
        let size: i64 = redis::cmd("MEMORY")
            .arg("usage")
            .arg(&key)
            .query(&mut con)?;
        let ttl: i64 = redis::cmd("TTL").arg(&key).query(&mut con)?;
        result = KeyValue::Hash(KV {
            key: key.to_string(),
            key_type: key_type.to_string(),
            value: value,
            size: size,
            ttl: ttl,
        });
    } else {
        result = KeyValue::Null(KV {
            key: key.to_string(),
            key_type: key_type.to_string(),
            value: "".to_string(),
            size: 0,
            ttl: 0,
        });
    }
    Ok(result)
}

#[tauri::command]
pub async fn get(conn: Connection, key: String, db: String) -> Response<KeyValue> {
    let res = handle_get(conn, key, db).await;
    match res {
        Ok(v) => Response::ok(Res::Success(v)),
        Err(e) => Response::error(e.to_string()),
    }
}

pub async fn handle_del(conn: Connection, key: String, db: String) -> redis::RedisResult<i32> {
    let mut con: redis::Connection = utils::get_connection(conn, db).await?;
    let res: i32 = redis::cmd("DEL").arg(&key).query(&mut con)?;
    Ok(res)
}

#[tauri::command]
pub async fn del(conn: Connection, key: String, db: String) -> Response<String> {
    let res = handle_del(conn, key, db).await;
    match res {
        Ok(v) => Response::ok(Res::Success(v.to_string())),
        Err(e) => Response::error(e.to_string()),
    }
}

pub async fn handle_expire(
    conn: Connection,
    key: String,
    ttl: i64,
    db: String,
) -> redis::RedisResult<i32> {
    let mut con: redis::Connection = utils::get_connection(conn, db).await?;
    let mut res: i32 = 0;
    if ttl > 0 {
        res = redis::cmd("EXPIRE").arg(&key).arg(ttl).query(&mut con)?;
    } else if ttl == -1 {
        res = redis::cmd("PERSIST").arg(&key).query(&mut con)?;
    }
    Ok(res)
}

#[tauri::command]
pub async fn expire(conn: Connection, key: String, ttl: i64, db: String) -> Response<String> {
    let res = handle_expire(conn, key, ttl, db).await;
    match res {
        Ok(v) => Response::ok(Res::Success(v.to_string())),
        Err(e) => Response::error(e.to_string()),
    }
}

pub async fn handle_info(conn: Connection, db: String) -> redis::RedisResult<String> {
    let mut con: redis::Connection = utils::get_connection(conn, db).await?;
    let res: String = redis::cmd("INFO").query(&mut con)?;
    Ok(res)
}

#[tauri::command]
pub async fn info(conn: Connection, db: String) -> Response<String> {
    let res = handle_info(conn, db).await;
    match res {
        Ok(v) => Response::ok(Res::Success(v)),
        Err(e) => Response::error(e.to_string()),
    }
}

#[tauri::command]
pub async fn exec(
    conn: Connection,
    command_lines: Vec<String>,
    db: String,
) -> Response<Vec<ExecResult>> {
    let con = utils::get_connection(conn, db).await;
    match con {
        Ok(mut con) => {
            let mut response: Vec<ExecResult> = Vec::new();
            for mut cmd_line in command_lines {
                if cmd_line.is_empty() {
                    continue;
                }

                if !cmd_line.ends_with("\n") {
                    cmd_line.push_str("\n");
                }

                let cmd_line_bytes = cmd_line.as_bytes();
                let request = con.req_packed_command(cmd_line_bytes);

                match request {
                    Ok(req) => match req {
                        Value::Nil => response.push(ExecResult {
                            command: cmd_line,
                            type_: "Nil".to_string(),
                            value: ExecValue::Nil,
                        }),
                        Value::Int(value) => response.push(ExecResult {
                            command: cmd_line,
                            type_: "Integer".to_string(),
                            value: ExecValue::Integer(value),
                        }),
                        Value::Data(value) => response.push(ExecResult {
                            command: cmd_line,
                            type_: "Data".to_string(),
                            value: ExecValue::Data(value),
                        }),
                        Value::Status(value) => response.push(ExecResult {
                            command: cmd_line,
                            type_: "Status".to_string(),
                            value: ExecValue::Status(value),
                        }),
                        Value::Okay => response.push(ExecResult {
                            command: cmd_line,
                            type_: "Okay".to_string(),
                            value: ExecValue::Okay,
                        }),
                        Value::Bulk(value) => response.push(ExecResult {
                            command: cmd_line,
                            type_: "Bulk".to_string(),
                            value: ExecValue::Bulk(resolve(value)),
                        }),
                    },
                    Err(err) => response.push(ExecResult {
                        command: cmd_line,
                        type_: "Error".to_string(),
                        value: ExecValue::Error(err.to_string()),
                    }),
                }
            }
            return Response::ok(Res::Success(response));
        }
        Err(e) => Response::ok(Res::Error(e.to_string())),
    }
}

fn resolve(value: Vec<Value>) -> Vec<ExecValue> {
    let mut resp: Vec<ExecValue> = Vec::new();
    for v in value {
        match v {
            Value::Nil => resp.push(ExecValue::Nil),
            Value::Int(value) => resp.push(ExecValue::Integer(value)),
            Value::Data(value) => resp.push(ExecValue::Data(value)),
            Value::Status(value) => resp.push(ExecValue::Status(value)),
            Value::Okay => resp.push(ExecValue::Okay),
            Value::Bulk(value) => resp.push(ExecValue::Bulk(resolve(value))),
        }
    }
    resp
}

#[tauri::command]
pub async fn test(conn: Connection, db: String) -> Response<bool> {
    let conn_str = format!(
        "redis://{}:{}@{}:{}/{}",
        "", conn.info.pass, conn.info.host, conn.info.port, db
    );

    let client_result = redis::Client::open(conn_str);
    match client_result {
        Ok(client) => {
            let con_result = client.get_connection();
            match con_result {
                Ok(_) => Response::ok(Res::Success(true)),
                Err(e) => Response::error(e.to_string()),
            }
        }
        Err(e) => Response::error(e.to_string()),
    }
}
