use futures_util::StreamExt;
use mongodb::{
    bson::{doc, Document},
    options::ClientOptions,
    Client,
};

use crate::entity::*;

#[tauri::command]
pub async fn databases(conn: Connection, database: String) -> Response<Vec<String>> {
    let conn_str = &format!(
        "mongodb://{}{}:{}{}",
        if conn.info.user.is_empty() {
            "".to_string()
        } else {
            format!("{}:{}@", conn.info.user, conn.info.pass)
        },
        conn.info.host,
        conn.info.port,
        if conn.info.db.is_empty() {
            if database.is_empty() {
                "".to_string()
            } else {
                format!("/{}", database)
            }
        } else {
            format!("/{}", conn.info.db)
        }
    );

    let client_result = Client::with_uri_str(conn_str).await;

    match client_result {
        Ok(client) => {
            let databases_result = client.list_database_names(None, None).await;
            match databases_result {
                Ok(databases) => Response::ok(Res::Success(databases)),
                Err(e) => Response::error(e.to_string()),
            }
        }
        Err(e) => Response::error(e.to_string()),
    }
}

#[tauri::command]
pub async fn collections(conn: Connection, database: String) -> Response<Vec<String>> {
    let conn_str = &format!(
        "mongodb://{}{}:{}{}",
        if conn.info.user.is_empty() {
            "".to_string()
        } else {
            format!("{}:{}@", conn.info.user, conn.info.pass)
        },
        conn.info.host,
        conn.info.port,
        if conn.info.db.is_empty() {
            if database.is_empty() {
                "".to_string()
            } else {
                format!("/{}", database)
            }
        } else {
            format!("/{}", conn.info.db)
        }
    );

    let client_result = Client::with_uri_str(conn_str).await;
    match client_result {
        Ok(client) => {
            let collections_result = client.database(&database).list_collection_names(None).await;
            match collections_result {
                Ok(collections) => Response::ok(Res::Success(collections)),
                Err(e) => Response::error(e.to_string()),
            }
        }
        Err(e) => Response::error(e.to_string()),
    }
}

#[tauri::command]
pub async fn documents(
    conn: Connection,
    database: String,
    collection: String,
    skip: i64,
    limit: i64,
    page: i64,
    size: i64,
    sorts: Vec<Sort>,
) -> Response<Document> {
    let conn_str = &format!(
        "mongodb://{}{}:{}{}",
        if conn.info.user.is_empty() {
            "".to_string()
        } else {
            format!("{}:{}@", conn.info.user, conn.info.pass)
        },
        conn.info.host,
        conn.info.port,
        if conn.info.db.is_empty() {
            if database.is_empty() {
                "".to_string()
            } else {
                format!("/{}", database)
            }
        } else {
            format!("/{}", conn.info.db)
        }
    );

    let client_result = Client::with_uri_str(conn_str).await;
    match client_result {
        Ok(client) => {
            let collection = client
                .database(&database)
                .collection::<Document>(&collection);
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

            let mut options_count = mongodb::options::CountOptions::default();
            options_count.max_time = Some(std::time::Duration::from_secs(10));
            let count = collection.count_documents(None, options_count).await;
            match count {
                Ok(count) => {
                    let mut options_find = mongodb::options::FindOptions::default();
                    options_find.skip = Some(skip_count as u64);
                    options_find.limit = Some(limit_count);
                    println!("sorts: {:?}", sorts);
                    let cursor = collection.find(None, options_find).await;
                    match cursor {
                        Ok(mut cursor) => {
                            let mut documents: Vec<Document> = Vec::new();
                            while let Some(result) = cursor.next().await {
                                match result {
                                    Ok(document) => documents.push(document),
                                    Err(e) => return Response::error(e.to_string()),
                                }
                            }
                            let mut res = Document::new();
                            res.insert("count", count as i64);
                            res.insert("documents", documents);
                            Response::ok(Res::Success(res))
                        }
                        Err(e) => Response::error(e.to_string()),
                    }
                }
                Err(e) => Response::error(e.to_string()),
            }
        }
        Err(e) => Response::error(e.to_string()),
    }
}

#[tauri::command]
pub async fn test(conn: Connection, database: String) -> Document {
    let conn_str = &format!(
        "mongodb://{}{}:{}{}",
        if conn.info.user.is_empty() {
            "".to_string()
        } else {
            format!("{}:{}@", conn.info.user, conn.info.pass)
        },
        conn.info.host,
        conn.info.port,
        if conn.info.db.is_empty() {
            "".to_string()
        } else {
            format!("/{}", conn.info.db)
        }
    );

    let client_options_result = ClientOptions::parse(conn_str).await;

    match client_options_result {
        Ok(client_options) => {
            let client_result = Client::with_options(client_options);
            match client_result {
                Ok(client) => {
                    let db = client.database(&database);
                    let res_result = db.run_command(doc! {"ping": 1}, None).await;
                    match res_result {
                        Ok(res) => {
                            doc! {"code": 10000, "msg": "success", "data": res}
                        }
                        Err(e) => {
                            doc! {"code": 40000, "msg": e.to_string(), "data": null}
                        }
                    }
                }
                Err(e) => {
                    doc! {"code": 40000, "msg": e.to_string(), "data": null}
                }
            }
        }
        Err(e) => {
            doc! {"code": 40000, "msg": e.to_string(), "data": null}
        }
    }
}
