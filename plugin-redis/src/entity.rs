use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

#[derive(Debug, Serialize, Deserialize)]
pub enum Res<T> {
    Null,
    Success(T),
    Error(String),
    Error5(String),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Response<T> {
    pub code: i32,
    pub msg: String,
    pub data: Res<T>,
}

impl<T> Response<T> {
    pub fn new(code: i32, msg: String, data: Res<T>) -> Self {
        Self { code, msg, data }
    }

    pub fn ok(data: Res<T>) -> Self {
        Self::new(10000, "OK".to_string(), data)
    }

    pub fn error(msg: String) -> Self {
        if msg.is_empty() {
            Self::new(40000, "Request Error.".to_string(), Res::Null)
        } else {
            Self::new(40000, msg, Res::Null)
        }
    }

    pub fn error5() -> Self {
        Self::new(50000, "Api Error.".to_string(), Res::Null)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KV<T> {
    pub key: String,
    pub key_type: String,
    pub value: T,
    pub size: i64,
    pub ttl: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum KeyValue {
    ReJson(KV<String>),
    String(KV<String>),
    List(KV<Vec<String>>),
    Set(KV<HashSet<String>>),
    Zset(KV<Vec<String>>),
    Hash(KV<HashMap<String, String>>),
    Null(KV<String>),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ZsetValue {
    pub score: f64,
    pub value: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HashValue {
    pub field: String,
    pub value: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Info {
    pub name: String,
    pub host: String,
    pub port: String,
    pub user: String,
    pub pass: String,
    pub index: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Connection {
    pub id: String,
    pub db_type: String,
    pub info: Info,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ExecValue {
    Nil,
    Okay,
    Data(Vec<u8>),
    Status(String),
    Integer(i64),
    Bulk(Vec<ExecValue>),
    Error(String),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExecResult {
    pub command: String,
    pub type_: String,
    pub value: ExecValue,
}
