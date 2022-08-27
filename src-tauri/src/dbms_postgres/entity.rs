use serde::{Deserialize, Serialize};
use std::{collections::HashMap, net::IpAddr};

#[derive(Debug, Serialize, Deserialize)]
pub struct Response<T> {
    pub code: i32,
    pub msg: String,
    pub data: Option<T>,
}

impl<T> Response<T> {
    pub fn new(code: i32, msg: String, data: Option<T>) -> Self {
        Self { code, msg, data }
    }

    pub fn ok(data: Option<T>) -> Self {
        Self::new(10000, "OK".to_string(), data)
    }

    pub fn error(code: i32, msg: String) -> Self {
        Self::new(code, msg, None)
    }

    pub fn err4() -> Self {
        Self::new(40000, "Request Error.".to_string(), None)
    }

    pub fn err5() -> Self {
        Self::new(50000, "Api Error.".to_string(), None)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KV<T> {
    pub key: String,
    pub value: T,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Field {
    Bool(KV<bool>),
    Char(KV<i8>),
    SmallInt(KV<i16>),
    SmallSerial(KV<i16>),
    Int(KV<i32>),
    Serial(KV<i32>),
    Oid(KV<u32>),
    BigInt(KV<i64>),
    BigSerial(KV<i64>),
    Real(KV<f32>),
    Double(KV<f64>),
    DoublePrecision(KV<f64>),
    VarChar(KV<String>),
    CharN(KV<String>),
    Text(KV<String>),
    Citext(KV<String>),
    Name(KV<String>),
    Unknown(KV<String>),
    Bytea(KV<Vec<u8>>),
    Hstore(KV<HashMap<String, Option<String>>>),
    Timestamp(KV<u64>),
    TimestampTZ(KV<u64>),
    Inet(KV<IpAddr>),

    // other
    Json(KV<String>),
    Xml(KV<String>),
}
