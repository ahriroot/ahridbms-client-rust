use postgres::Error;
use std::time::{SystemTime, UNIX_EPOCH};
use tokio_postgres::{types::Type, NoTls};

use crate::entity::*;

#[tauri::command]
pub async fn execsql_select(conn: Connection, sql: &str) -> Result<Vec<Vec<Field>>, Error> {
    let conn_str = &format!(
        "postgres://{}{}{}@{}{}{}{}{}",
        conn.info.user,
        if !conn.info.pass.is_empty() { ":" } else { "" },
        conn.info.pass,
        conn.info.host,
        if !conn.info.port.is_empty() { ":" } else { "" },
        conn.info.port,
        if !conn.info.db.is_empty() { "/" } else { "" },
        conn.info.db
    );

    let (client, connection) = tokio_postgres::connect(conn_str, NoTls).await?;
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });
    let rows = client.query(sql, &[]).await?;
    let mut result_data: Vec<Vec<Field>> = Vec::new();
    for row in rows.iter() {
        let mut result_row: Vec<Field> = Vec::new();
        let columns = row.columns();
        for column in columns {
            let name = column.name();
            let typ = column.type_();

            match *typ {
                Type::BOOL => {
                    let data: Option<bool> = row.get(name);
                    result_row.push(Field::Bool(KV {
                        key: name.to_string(),
                        value: data,
                    }));
                }
                Type::CHAR => {
                    let data: Option<i8> = row.get(name);
                    result_row.push(Field::Char(KV {
                        key: name.to_string(),
                        value: data,
                    }));
                }
                Type::INT2 => {
                    let data: Option<i16> = row.get(name);
                    result_row.push(Field::SmallInt(KV {
                        key: name.to_string(),
                        value: data,
                    }));
                }
                Type::INT4 => {
                    let data: Option<i32> = row.get(name);
                    result_row.push(Field::Int(KV {
                        key: name.to_string(),
                        value: data,
                    }));
                }
                Type::INT8 => {
                    let data: Option<i64> = row.get(name);
                    result_row.push(Field::BigInt(KV {
                        key: name.to_string(),
                        value: data,
                    }));
                }
                Type::TEXT => {
                    let data: Option<String> = row.get(name);
                    result_row.push(Field::Text(KV {
                        key: name.to_string(),
                        value: data,
                    }));
                }
                Type::OID => {
                    let data: Option<u32> = row.get(name);
                    result_row.push(Field::Oid(KV {
                        key: name.to_string(),
                        value: data,
                    }));
                }
                Type::FLOAT4 => {
                    let data: Option<f32> = row.get(name);
                    result_row.push(Field::Real(KV {
                        key: name.to_string(),
                        value: data,
                    }));
                }
                Type::FLOAT8 => {
                    let data: Option<f64> = row.get(name);
                    result_row.push(Field::Double(KV {
                        key: name.to_string(),
                        value: data,
                    }));
                }
                Type::TIMESTAMPTZ => {
                    let data: Option<SystemTime> = row.get(name);
                    if let Some(data) = data {
                        let timestamp = data.duration_since(UNIX_EPOCH).unwrap().as_secs();
                        result_row.push(Field::TimestampTZ(KV {
                            key: name.to_string(),
                            value: Some(timestamp),
                        }));
                    } else {
                        result_row.push(Field::TimestampTZ(KV {
                            key: name.to_string(),
                            value: None,
                        }));
                    }
                }
                Type::TIMESTAMP => {
                    let data: Option<SystemTime> = row.get(name);
                    if let Some(data) = data {
                        let timestamp = data.duration_since(UNIX_EPOCH).unwrap().as_secs();
                        result_row.push(Field::Timestamp(KV {
                            key: name.to_string(),
                            value: Some(timestamp),
                        }));
                    } else {
                        result_row.push(Field::Timestamp(KV {
                            key: name.to_string(),
                            value: None,
                        }));
                    }
                }
                Type::NAME => {
                    let data: Option<String> = row.get(name);
                    result_row.push(Field::Name(KV {
                        key: name.to_string(),
                        value: data,
                    }));
                }
                Type::XID => {
                    result_row.push(Field::Ignore(KV {
                        key: name.to_string(),
                        value: None,
                    }));
                }
                Type::ACLITEM => {
                    result_row.push(Field::Ignore(KV {
                        key: name.to_string(),
                        value: None,
                    }));
                }
                Type::ACLITEM_ARRAY => {
                    result_row.push(Field::Ignore(KV {
                        key: name.to_string(),
                        value: None,
                    }));
                }
                _ => {}
            }
        }
        result_data.push(result_row);
    }
    Ok(result_data)
}

#[tauri::command]
pub async fn get_databases(conn: Connection) -> Response<Vec<Vec<Field>>> {
    let res = execsql_select(
        conn,
        "SELECT * FROM pg_database WHERE datistemplate = false;",
    )
    .await;

    match res {
        Ok(v) => Response::ok(Res::Success(v)),
        Err(e) => Response::error(e.to_string()),
    }
}

#[tauri::command]
pub async fn get_tables(mut conn: Connection, database: String) -> Response<Vec<Vec<Field>>> {
    conn.info.db = database;
    let res = execsql_select(conn, "SELECT * FROM pg_tables WHERE schemaname = 'public';").await;

    match res {
        Ok(v) => Response::ok(Res::Success(v)),
        Err(e) => Response::error(e.to_string()),
    }
}

#[tauri::command]
pub async fn get_columns(
    mut conn: Connection,
    database: String,
    table: String,
) -> Response<Vec<Vec<Field>>> {
    conn.info.db = database;
    let sql = &format!(
        "SELECT a.attnum, a.attname AS field, t.typname AS type, a.attlen AS length, a.atttypmod AS lengthvar, a.attnotnull AS notnull
        FROM pg_class c, pg_attribute a, pg_type t
        WHERE c.relname = '{}' AND a.attnum > 0 AND a.attrelid = c.oid AND a.atttypid = t.oid
        ORDER BY a.attnum;",
        table
    );
    
    let res = execsql_select(conn, sql).await;

    match res {
        Ok(v) => Response::ok(Res::Success(v)),
        Err(e) => Response::error(e.to_string()),
    }
}

#[tauri::command]
pub async fn select(
    conn: Connection,
    skip: i64,
    limit: i64,
    page: i64,
    size: i64,
    table: String,
) -> Response<Vec<Vec<Field>>> {
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
    let sql = &format!(
        "SELECT * FROM public.\"{}\" LIMIT {} OFFSET {};",
        table, limit_count, skip_count
    );
    let res = execsql_select(conn, sql).await;
    match res {
        Ok(v) => Response::ok(Res::Success(v)),
        Err(e) => Response::error(e.to_string()),
    }
}
