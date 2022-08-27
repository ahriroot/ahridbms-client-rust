use std::time::{SystemTime, UNIX_EPOCH};
use tokio_postgres::{types::Type, NoTls};

use crate::dbms_postgres::entity::*;

#[tauri::command]
pub async fn select(
    skip: i64,
    limit: i64,
    page: i64,
    size: i64,
    table: String,
) -> Response<Vec<Vec<Field>>> {
    let user = "postgres".to_string();
    let pass = "Aa12345.".to_string();
    let host = "127.0.0.1".to_string();
    let port = "5432".to_string();
    let db = "ahriknow".to_string();

    let conn_str = &format!(
        "postgres://{}{}{}@{}{}{}{}{}",
        user,
        if !pass.is_empty() { ":" } else { "" },
        pass,
        host,
        if !port.is_empty() { ":" } else { "" },
        port,
        if !db.is_empty() { "/" } else { "" },
        db
    );

    let mut skip_count = skip;
    let mut limit_count = limit;
    if page > 0 && size > 0 {
        skip_count = (page - 1) * size;
        limit_count = size;
    }
    if skip_count < 0 {
        skip_count = 0;
    }
    if limit_count < 0 {
        limit_count = 1;
    }
    let (client, connection) = tokio_postgres::connect(conn_str, NoTls).await.unwrap();
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });
    let sql = &format!(
        "SELECT * FROM public.\"{}\" LIMIT {} OFFSET {}",
        table, limit_count, skip_count
    );
    let result = client.query(sql, &[]).await;
    let mut result_data: Vec<Vec<Field>> = Vec::new();
    match result {
        Ok(rows) => {
            for row in rows.iter() {
                let mut result_row: Vec<Field> = Vec::new();
                let columns = row.columns();
                for column in columns {
                    let name = column.name();
                    let typ = column.type_();
                    match *typ {
                        Type::BOOL => {
                            let data: bool = row.get(name);
                            result_row.push(Field::Bool(KV {
                                key: name.to_string(),
                                value: data,
                            }));
                        }
                        Type::CHAR => {
                            let data: i8 = row.get(name);
                            result_row.push(Field::Char(KV {
                                key: name.to_string(),
                                value: data,
                            }));
                        }
                        Type::INT2 => {
                            let data: i16 = row.get(name);
                            result_row.push(Field::SmallInt(KV {
                                key: name.to_string(),
                                value: data,
                            }));
                        }
                        Type::INT4 => {
                            let data: i32 = row.get(name);
                            result_row.push(Field::Int(KV {
                                key: name.to_string(),
                                value: data,
                            }));
                        }
                        Type::INT8 => {
                            let data: i64 = row.get(name);
                            result_row.push(Field::BigInt(KV {
                                key: name.to_string(),
                                value: data,
                            }));
                        }
                        Type::TEXT => {
                            let data: String = row.get(name);
                            result_row.push(Field::Text(KV {
                                key: name.to_string(),
                                value: data,
                            }));
                        }
                        Type::OID => {
                            let data: u32 = row.get(name);
                            result_row.push(Field::Oid(KV {
                                key: name.to_string(),
                                value: data,
                            }));
                        }
                        Type::FLOAT4 => {
                            let data: f32 = row.get(name);
                            result_row.push(Field::Real(KV {
                                key: name.to_string(),
                                value: data,
                            }));
                        }
                        Type::FLOAT8 => {
                            let data: f64 = row.get(name);
                            result_row.push(Field::Double(KV {
                                key: name.to_string(),
                                value: data,
                            }));
                        }
                        Type::TIMESTAMPTZ => {
                            let data: SystemTime = row.get(name);
                            let timestamp = data.duration_since(UNIX_EPOCH).unwrap().as_secs();
                            result_row.push(Field::TimestampTZ(KV {
                                key: name.to_string(),
                                value: timestamp,
                            }));
                        }
                        Type::TIMESTAMP => {
                            let data: SystemTime = row.get(name);
                            let timestamp = data.duration_since(UNIX_EPOCH).unwrap().as_secs();
                            result_row.push(Field::Timestamp(KV {
                                key: name.to_string(),
                                value: timestamp,
                            }));
                        }
                        _ => {}
                    }
                }
                result_data.push(result_row);
            }
        }
        Err(e) => return Response::error(40000, e.to_string()),
    }
    Response::ok(Some(result_data))
}
