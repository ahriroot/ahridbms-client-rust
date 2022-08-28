#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use ahridbms;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut app = tauri::Builder::default();
    app = app.invoke_handler(tauri::generate_handler![
        // redis
        ahridbms::dbms_redis::api::keys,
        ahridbms::dbms_redis::api::get,
        ahridbms::dbms_redis::api::del,
        ahridbms::dbms_redis::api::expire,
        ahridbms::dbms_redis::api::key_space,
        ahridbms::dbms_redis::api::set_string,
        ahridbms::dbms_redis::api::rpush,
        ahridbms::dbms_redis::api::sadd,
        ahridbms::dbms_redis::api::zadd,
        ahridbms::dbms_redis::api::hmset,

        // postgres
        ahridbms::dbms_postgres::api::pg_select,
        ahridbms::dbms_postgres::api::pg_get_databases,
    ]);
    app.run(tauri::generate_context!())
        .expect("error while running ahridbms application");
    Ok(())
}
