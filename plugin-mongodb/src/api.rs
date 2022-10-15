use futures_util::StreamExt;
use mongodb::{
    bson::{doc, Document},
    options::ClientOptions,
    Client, Collection,
};

use crate::entity::*;

#[tauri::command]
pub async fn find() -> Vec<Document> {
    let mut client_options = ClientOptions::parse("mongodb://root:Aa12345.@localhost:27017")
        .await
        .unwrap();

    // Manually set an option.
    client_options.app_name = Some("AhriDMBS App".to_string());

    // Get a handle to the deployment.
    let client = Client::with_options(client_options).unwrap();

    let test_db = client.database("test");
    let user_collection: Collection<Document> = test_db.collection("user");

    let mut users_cursor = user_collection.find(None, None).await.unwrap();

    let mut users = Vec::new();
    while let Some(result) = users_cursor.next().await {
        users.push(result.unwrap());
    }

    users
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
