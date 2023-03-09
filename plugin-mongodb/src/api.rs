use futures_util::StreamExt;
use mongodb::{
    bson::{doc, Bson, Document},
    options::{
        ClientOptions, DeleteOptions, FindOptions, InsertManyOptions, InsertOneOptions,
        UpdateOptions, WriteConcern,
    },
    Client,
};

use crate::entity::*;

async fn get_conn_str(conn: Connection) -> String {
    format!(
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
    )
}

#[tauri::command]
pub async fn databases(conn: Connection) -> Response<Vec<String>> {
    let conn_str = get_conn_str(conn).await;

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
    let conn_str = get_conn_str(conn).await;

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
    let conn_str = get_conn_str(conn).await;

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
                    let mut sort_doc = Document::new();
                    for sort in sorts {
                        if sort.order == "ASC" {
                            sort_doc.insert(sort.field, 1);
                        } else if sort.order == "DESC" {
                            sort_doc.insert(sort.field, -1);
                        }
                    }
                    options_find.sort = Some(sort_doc);
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
pub async fn drop_database(conn: Connection, database: String) -> Response<Document> {
    let conn_str = get_conn_str(conn).await;

    let client_result = Client::with_uri_str(conn_str).await;
    match client_result {
        Ok(client) => {
            let drop_result = client.database(&database).drop(None).await;
            match drop_result {
                Ok(_) => Response::ok(Res::Success(doc! {"drop_result": "success"})),
                Err(e) => Response::error(e.to_string()),
            }
        }
        Err(e) => Response::error(e.to_string()),
    }
}

#[tauri::command]
pub async fn drop_collection(
    conn: Connection,
    database: String,
    collection: String,
) -> Response<Document> {
    let conn_str = get_conn_str(conn).await;

    let client_result = Client::with_uri_str(conn_str).await;
    match client_result {
        Ok(client) => {
            let collection = client
                .database(&database)
                .collection::<Document>(&collection);
            let drop_result = collection.drop(None).await;
            match drop_result {
                Ok(_) => Response::ok(Res::Success(doc! {"drop_result": "success"})),
                Err(e) => Response::error(e.to_string()),
            }
        }
        Err(e) => Response::error(e.to_string()),
    }
}

#[tauri::command]
pub async fn insert_one(
    conn: Connection,
    database: String,
    collection: String,
    document: Document,
    options: Document,
) -> Response<Document> {
    let conn_str = get_conn_str(conn).await;

    let client_result = Client::with_uri_str(conn_str).await;
    match client_result {
        Ok(client) => {
            let collection = client
                .database(&database)
                .collection::<Document>(&collection);

            let write_concern = bson::from_bson::<WriteConcern>(Bson::Document(options)).unwrap();
            let options = InsertOneOptions::builder()
                .write_concern(write_concern)
                .build();
            let insert_result = collection.insert_one(document, options).await;
            match insert_result {
                Ok(insert_result) => {
                    let mut res = Document::new();
                    res.insert("inserted_id", insert_result.inserted_id);
                    Response::ok(Res::Success(res))
                }
                Err(e) => Response::error(e.to_string()),
            }
        }
        Err(e) => Response::error(e.to_string()),
    }
}

#[tauri::command]
pub async fn insert_many(
    conn: Connection,
    database: String,
    collection: String,
    document: Vec<Document>,
    options: Document,
) -> Response<Document> {
    let conn_str = get_conn_str(conn).await;

    let client_result = Client::with_uri_str(conn_str).await;
    match client_result {
        Ok(client) => {
            let collection = client
                .database(&database)
                .collection::<Document>(&collection);

            let write_concern = bson::from_bson::<WriteConcern>(Bson::Document(options)).unwrap();
            let options = InsertManyOptions::builder()
                .write_concern(write_concern)
                .build();
            let insert_result = collection.insert_many(document, options).await;
            match insert_result {
                Ok(insert_result) => {
                    let mut res = Document::new();
                    // 遍历
                    let mut inserted_ids = Vec::new();
                    for (_, v) in insert_result.inserted_ids {
                        inserted_ids.push(v);
                    }
                    res.insert("acknowledged", "");
                    res.insert("inserted_ids", inserted_ids);
                    Response::ok(Res::Success(res))
                }
                Err(e) => Response::error(e.to_string()),
            }
        }
        Err(e) => Response::error(e.to_string()),
    }
}

#[tauri::command]
pub async fn update_one(
    conn: Connection,
    database: String,
    collection: String,
    filter: Document,
    update: Document,
    options: Document,
) -> Response<Document> {
    let conn_str = get_conn_str(conn).await;

    let client_result = Client::with_uri_str(conn_str).await;
    match client_result {
        Ok(client) => {
            let collection = client
                .database(&database)
                .collection::<Document>(&collection);

            let write_concern = bson::from_bson::<WriteConcern>(Bson::Document(options)).unwrap();
            let options = UpdateOptions::builder()
                .write_concern(write_concern)
                .build();
            let update_result = collection.update_one(filter, update, options).await;
            match update_result {
                Ok(update_result) => {
                    let mut res = Document::new();
                    res.insert("matched_count", update_result.matched_count as i64);
                    res.insert("modified_count", update_result.modified_count as i64);
                    res.insert("upserted_id", update_result.upserted_id);
                    Response::ok(Res::Success(res))
                }
                Err(e) => Response::error(e.to_string()),
            }
        }
        Err(e) => Response::error(e.to_string()),
    }
}

#[tauri::command]
pub async fn update_many(
    conn: Connection,
    database: String,
    collection: String,
    filter: Document,
    update: Document,
    options: Document,
) -> Response<Document> {
    let conn_str = get_conn_str(conn).await;

    let client_result = Client::with_uri_str(conn_str).await;
    match client_result {
        Ok(client) => {
            let collection = client
                .database(&database)
                .collection::<Document>(&collection);

            let write_concern = bson::from_bson::<WriteConcern>(Bson::Document(options)).unwrap();
            let options = UpdateOptions::builder()
                .write_concern(write_concern)
                .build();
            let update_result = collection.update_many(filter, update, options).await;
            match update_result {
                Ok(update_result) => {
                    let mut res = Document::new();
                    res.insert("matched_count", update_result.matched_count as i64);
                    res.insert("modified_count", update_result.modified_count as i64);
                    res.insert("upserted_id", update_result.upserted_id);
                    Response::ok(Res::Success(res))
                }
                Err(e) => Response::error(e.to_string()),
            }
        }
        Err(e) => Response::error(e.to_string()),
    }
}

#[tauri::command]
pub async fn delete_one(
    conn: Connection,
    database: String,
    collection: String,
    document: Document,
    options: Document,
) -> Response<Document> {
    let conn_str = get_conn_str(conn).await;

    let client_result = Client::with_uri_str(conn_str).await;
    match client_result {
        Ok(client) => {
            let collection = client
                .database(&database)
                .collection::<Document>(&collection);

            let write_concern = bson::from_bson::<WriteConcern>(Bson::Document(options)).unwrap();
            let options = DeleteOptions::builder()
                .write_concern(write_concern)
                .build();
            let delete_result = collection.delete_one(document, options).await;
            match delete_result {
                Ok(delete_result) => {
                    let mut res = Document::new();
                    res.insert("acknowledged", "");
                    res.insert("deleted_count", delete_result.deleted_count as i64);
                    Response::ok(Res::Success(res))
                }
                Err(e) => Response::error(e.to_string()),
            }
        }
        Err(e) => Response::error(e.to_string()),
    }
}

#[tauri::command]
pub async fn delete_many(
    conn: Connection,
    database: String,
    collection: String,
    document: Document,
    options: Document,
) -> Response<Document> {
    let conn_str = get_conn_str(conn).await;

    let client_result = Client::with_uri_str(conn_str).await;
    match client_result {
        Ok(client) => {
            let collection = client
                .database(&database)
                .collection::<Document>(&collection);

            let write_concern = bson::from_bson::<WriteConcern>(Bson::Document(options)).unwrap();
            let options = DeleteOptions::builder()
                .write_concern(write_concern)
                .build();
            let delete_result = collection.delete_many(document, options).await;
            match delete_result {
                Ok(delete_result) => {
                    let mut res = Document::new();
                    res.insert("acknowledged", "");
                    res.insert("deleted_count", delete_result.deleted_count as i64);
                    Response::ok(Res::Success(res))
                }
                Err(e) => Response::error(e.to_string()),
            }
        }
        Err(e) => Response::error(e.to_string()),
    }
}

#[tauri::command]
pub async fn find(
    conn: Connection,
    database: String,
    collection: String,
    document: Document,
    options: Document,
) -> Response<Vec<Document>> {
    let conn_str = get_conn_str(conn).await;

    let client_result = Client::with_uri_str(conn_str).await;
    match client_result {
        Ok(client) => {
            let collection = client
                .database(&database)
                .collection::<Document>(&collection);

            let find_options = bson::from_bson::<FindOptions>(Bson::Document(options)).unwrap();
            let find_result = collection.find(document, find_options).await;
            match find_result {
                Ok(find_result) => {
                    let mut res = Vec::new();
                    let r = find_result.collect::<Vec<_>>().await;
                    for i in r {
                        res.push(i.unwrap());
                    }
                    Response::ok(Res::Success(res))
                }
                Err(e) => Response::error(e.to_string()),
            }
        }
        Err(e) => Response::error(e.to_string()),
    }
}

#[tauri::command]
pub async fn test(conn: Connection, database: String) -> Document {
    let conn_str = get_conn_str(conn).await;

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
