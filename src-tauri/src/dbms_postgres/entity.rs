use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Response<T> {
    pub code: i32,
    pub msg: String,
    pub data: T,
}

impl<T> Response<T> {
    pub fn new(code: i32, msg: String, data: T) -> Self {
        Self { code, msg, data }
    }

    pub fn ok(data: T) -> Self {
        Self::new(10000, "OK".to_string(), data)
    }

    pub fn err4(data: T) -> Self {
        Self::new(40000, "Request Error.".to_string(), data)
    }

    pub fn err5(data: T) -> Self {
        Self::new(50000, "Api Error.".to_string(), data)
    }
}
/*
[
    [
        {
            "name": "id",
            "typ": "int",
            "value": 1
        },
        {
            "name": "text",
            "typ": "string",
            "value": "text_value"
        }
    ]
]
*/

#[derive(Debug, Serialize, Deserialize)]
pub struct KV<T> {
    pub key: String,
    pub value: T,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Field {
    Bool(KV<bool>),
    String(KV<String>),
}
