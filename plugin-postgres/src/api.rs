use chrono::{NaiveDate, NaiveTime};
use postgres::Error;
use std::time::{SystemTime, UNIX_EPOCH};
use tokio_postgres::{types::Type, NoTls};

use crate::entity::*;

pub async fn get_connection(conn: Connection) -> Result<tokio_postgres::Client, Error> {
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

    Ok(client)
}

pub async fn execsql_select(conn: Connection, sql: &str) -> Result<Vec<Vec<Field>>, Error> {
    let client = get_connection(conn).await?;
    let rows = client.query(sql, &[]).await?;
    let mut result_data: Vec<Vec<Field>> = Vec::new();
    for row in rows.iter() {
        let mut result_row: Vec<Field> = Vec::new();
        let columns = row.columns();
        for column in columns {
            let name = column.name();
            let typ = column.type_();
            let typename = typ.name();

            match *typ {
                Type::BOOL => {
                    let data: Option<bool> = row.get(name);
                    result_row.push(Field::Bool(KV {
                        key: name.to_string(),
                        typename: typename.to_string(),
                        value: data,
                    }));
                }
                Type::CHAR => {
                    let data: Option<i8> = row.get(name);
                    result_row.push(Field::Char(KV {
                        key: name.to_string(),
                        typename: typename.to_string(),
                        value: data,
                    }));
                }
                Type::VARCHAR => {
                    let data: Option<String> = row.get(name);
                    result_row.push(Field::VarChar(KV {
                        key: name.to_string(),
                        typename: typename.to_string(),
                        value: data,
                    }));
                }
                Type::INT2 => {
                    let data: Option<i16> = row.get(name);
                    result_row.push(Field::SmallInt(KV {
                        key: name.to_string(),
                        typename: typename.to_string(),
                        value: data,
                    }));
                }
                Type::INT4 => {
                    let data: Option<i32> = row.get(name);
                    result_row.push(Field::Int(KV {
                        key: name.to_string(),
                        typename: typename.to_string(),
                        value: data,
                    }));
                }
                Type::INT8 => {
                    let data: Option<i64> = row.get(name);
                    result_row.push(Field::BigInt(KV {
                        key: name.to_string(),
                        typename: typename.to_string(),
                        value: data,
                    }));
                }
                Type::TEXT => {
                    let data: Option<String> = row.get(name);
                    result_row.push(Field::Text(KV {
                        key: name.to_string(),
                        typename: typename.to_string(),
                        value: data,
                    }));
                }
                Type::OID => {
                    let data: Option<u32> = row.get(name);
                    result_row.push(Field::Oid(KV {
                        key: name.to_string(),
                        typename: typename.to_string(),
                        value: data,
                    }));
                }
                Type::FLOAT4 => {
                    let data: Option<f32> = row.get(name);
                    result_row.push(Field::Real(KV {
                        key: name.to_string(),
                        typename: typename.to_string(),
                        value: data,
                    }));
                }
                Type::FLOAT8 => {
                    let data: Option<f64> = row.get(name);
                    result_row.push(Field::Double(KV {
                        key: name.to_string(),
                        typename: typename.to_string(),
                        value: data,
                    }));
                }
                Type::TIMESTAMPTZ => {
                    let data: Option<SystemTime> = row.get(name);
                    if let Some(data) = data {
                        let timestamp = data.duration_since(UNIX_EPOCH).unwrap().as_secs();
                        result_row.push(Field::TimestampTZ(KV {
                            key: name.to_string(),
                            typename: typename.to_string(),
                            value: Some(timestamp),
                        }));
                    } else {
                        result_row.push(Field::TimestampTZ(KV {
                            key: name.to_string(),
                            typename: typename.to_string(),
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
                            typename: typename.to_string(),
                            value: Some(timestamp),
                        }));
                    } else {
                        result_row.push(Field::Timestamp(KV {
                            key: name.to_string(),
                            typename: typename.to_string(),
                            value: None,
                        }));
                    }
                }
                Type::DATE => {
                    let data: Option<NaiveDate> = row.get(name);
                    let res: String = match data {
                        Some(data) => data.format("%Y-%m-%d").to_string(),
                        None => "".to_string(),
                    };
                    result_row.push(Field::Date(KV {
                        key: name.to_string(),
                        typename: typename.to_string(),
                        value: Some(res),
                    }));
                }
                Type::TIME => {
                    let data: Option<NaiveTime> = row.get(name);
                    let res: String = match data {
                        Some(data) => data.format("%H:%M:%S").to_string(),
                        None => "".to_string(),
                    };
                    result_row.push(Field::Time(KV {
                        key: name.to_string(),
                        typename: typename.to_string(),
                        value: Some(res),
                    }));
                }
                Type::NAME => {
                    let data: Option<String> = row.get(name);
                    result_row.push(Field::Name(KV {
                        key: name.to_string(),
                        typename: typename.to_string(),
                        value: data,
                    }));
                }
                Type::XID => {
                    result_row.push(Field::Ignore(KV {
                        key: name.to_string(),
                        typename: typename.to_string(),
                        value: None,
                    }));
                }
                Type::ACLITEM => {
                    result_row.push(Field::Ignore(KV {
                        key: name.to_string(),
                        typename: typename.to_string(),
                        value: None,
                    }));
                }
                Type::ACLITEM_ARRAY => {
                    result_row.push(Field::Ignore(KV {
                        key: name.to_string(),
                        typename: typename.to_string(),
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
    let columns_result = get_columns(
        conn.clone(),
        "postgres".to_string(),
        "pg_database".to_string(),
    )
    .await;

    let columns: Vec<Vec<Field>> = match columns_result.data {
        Res::Null => return Response::error("No data".to_string()),
        Res::Success(c) => c,
        Res::Error(e) => return Response::error(e),
        Res::Error5(e) => return Response::error(e),
    };

    let mut column_names = Vec::new();
    for cols in columns {
        for col in cols {
            match col {
                Field::Name(KV { key, value, .. }) => {
                    if key == "field" {
                        column_names.push(value.unwrap());
                        break;
                    }
                }
                _ => {}
            }
        }
    }

    let columns_str = column_names.join(", ");

    let res = execsql_select(
        conn,
        &format!(
            "SELECT {} FROM pg_database WHERE datistemplate = false;",
            columns_str,
        ),
    )
    .await;

    let r = match res {
        Ok(v) => Response::ok(Res::Success(v)),
        Err(e) => Response::error(e.to_string()),
    };
    r
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
pub async fn get_primary_keys(
    mut conn: Connection,
    database: String,
    table: String,
) -> Response<Vec<Vec<Field>>> {
    conn.info.db = database;
    let sql = &format!(
        "SELECT
            pg_constraint.conname AS pk_name,
            pg_attribute.attname AS colname,
            pg_type.typname AS typename 
        FROM
            pg_constraint
            INNER JOIN pg_class ON pg_constraint.conrelid = pg_class.oid
            INNER JOIN pg_attribute ON pg_attribute.attrelid = pg_class.oid 
            AND pg_attribute.attnum = ANY (pg_constraint.conkey)
            INNER JOIN pg_type ON pg_type.oid = pg_attribute.atttypid 
        WHERE
            pg_class.relname = '{}' 
            AND pg_constraint.contype = 'p';",
        table
    );

    let res = execsql_select(conn, sql).await;

    match res {
        Ok(v) => Response::ok(Res::Success(v)),
        Err(e) => Response::error(e.to_string()),
    }
}

#[tauri::command]
pub async fn get_table_struct(
    mut conn: Connection,
    database: String,
    table: String,
) -> Response<Vec<Vec<Field>>> {
    conn.info.db = database;
    let sql = &format!(
        "SELECT a.attnum, a.attname, t.typname, a.attlen, a.attnotnull
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
    mut conn: Connection,
    skip: i64,
    limit: i64,
    page: i64,
    size: i64,
    sorts: Vec<Sort>,
    database: String,
    table: String,
) -> Response<SelectResult> {
    conn.info.db = database;
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
    let mut sql = format!(
        "SELECT * FROM public.\"{}\" LIMIT {} OFFSET {};",
        table, limit_count, skip_count
    );
    if sorts.len() > 0 {
        let sort_array: Vec<String> = sorts
            .iter()
            .map(|s| format!("{} {}", s.field, s.order))
            .collect();
        let sort = sort_array.join(", ");
        sql = format!(
            "SELECT * FROM public.\"{}\" ORDER BY {} LIMIT {} OFFSET {};",
            table, sort, limit_count, skip_count
        );
    }
    let res = execsql_select(conn.clone(), &sql).await;
    let count = execsql_select(conn, &format!("SELECT COUNT(*) FROM public.\"{}\";", table)).await;
    match (res, count) {
        (Ok(v), Ok(c)) => {
            let f = &c[0][0];
            let count: i64 = match f {
                Field::BigInt(v) => match v.value {
                    Some(v) => v,
                    None => 0,
                },
                _ => 0,
            };
            Response::ok(Res::Success(SelectResult { data: v, count }))
        }
        (Err(e), _) => Response::error(e.to_string()),
        (_, Err(e)) => Response::error(e.to_string()),
    }
}

pub async fn execsql_update(conn: Connection, sql: &str) -> Result<u64, Error> {
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

    let count = client.execute(sql, &[]).await?;
    Ok(count)
}

#[tauri::command]
pub async fn update(mut conn: Connection, database: String, sql: String) -> Response<u64> {
    conn.info.db = database;

    let res = execsql_update(conn, &sql).await;

    match res {
        Ok(v) => Response::ok(Res::Success(v)),
        Err(e) => Response::error(e.to_string()),
    }
}

async fn handle_execute_with_transaction(
    conn: Connection,
    sqls: Vec<String>,
) -> Result<u64, Error> {
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

    let (mut client, connection) = tokio_postgres::connect(conn_str, NoTls).await?;
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    let tx = client.transaction().await?;
    let mut count = 0;
    for sql in sqls {
        count += tx.execute(&sql, &[]).await?;
    }
    tx.commit().await?;
    Ok(count)
}

#[tauri::command]
pub async fn execute_with_transaction(
    mut conn: Connection,
    database: String,
    sqls: Vec<String>,
) -> Response<u64> {
    conn.info.db = database;

    let res = handle_execute_with_transaction(conn, sqls).await;

    match res {
        Ok(v) => Response::ok(Res::Success(v)),
        Err(e) => Response::error(e.to_string()),
    }
}

#[tauri::command]
pub async fn execute_select_sql(
    mut conn: Connection,
    database: String,
    sql: String,
) -> Response<Vec<Vec<Field>>> {
    conn.info.db = database;
    let res = execsql_select(conn.clone(), &sql).await;
    match res {
        Ok(v) => Response::ok(Res::Success(v)),
        Err(e) => Response::error(e.to_string()),
    }
}

#[tauri::command]
pub async fn test(conn: Connection, database: String) -> Response<bool> {
    let conn_str = &format!(
        "postgres://{}{}{}@{}{}{}{}{}",
        conn.info.user,
        if !conn.info.pass.is_empty() { ":" } else { "" },
        conn.info.pass,
        conn.info.host,
        if !conn.info.port.is_empty() { ":" } else { "" },
        conn.info.port,
        if !database.is_empty() { "/" } else { "" },
        database
    );
    let res = tokio_postgres::connect(conn_str, NoTls).await;
    match res {
        Ok(_) => Response::ok(Res::Success(true)),
        Err(e) => Response::error(e.to_string()),
    }
}

#[tauri::command]
pub async fn select_with_struct(
    conn: Connection,
    skip: i64,
    limit: i64,
    page: i64,
    size: i64,
    sorts: Vec<Sort>,
    database: String,
    table: String,
) -> Response<SelectWithStruct> {
    let table_primary = get_primary_keys(conn.clone(), database.clone(), table.clone()).await;
    let table_struct = get_table_struct(conn.clone(), database.clone(), table.clone()).await;
    let table_data = select(conn, skip, limit, page, size, sorts, database, table).await;

    if table_primary.code != 10000 {
        return Response::error(table_primary.msg);
    }

    if table_struct.code != 10000 {
        return Response::error(table_struct.msg);
    }

    if table_data.code != 10000 {
        return Response::error(table_data.msg);
    }

    let table_primary = match table_primary.data {
        Res::Success(v) => v,
        Res::Error(e) => return Response::error(e),
        Res::Error5(_) => return Response::error5(),
        Res::Null => return Response::error("Null".to_string()),
    };

    let table_struct = match table_struct.data {
        Res::Success(v) => v,
        Res::Error(e) => return Response::error(e),
        Res::Error5(_) => return Response::error5(),
        Res::Null => return Response::error("Null".to_string()),
    };

    let table_data = match table_data.data {
        Res::Success(v) => v,
        Res::Error(e) => return Response::error(e),
        Res::Error5(_) => return Response::error5(),
        Res::Null => return Response::error("Null".to_string()),
    };

    Response::ok(Res::Success(SelectWithStruct {
        table_primary,
        table_struct,
        table_data,
    }))
}
