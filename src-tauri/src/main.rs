#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use ahridbms::dbms_redis;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut app = tauri::Builder::default();
    app = app.invoke_handler(tauri::generate_handler![
        dbms_redis::api::keys,
        dbms_redis::api::get,
        dbms_redis::api::del,
        dbms_redis::api::expire,
        dbms_redis::api::key_space,
        dbms_redis::api::set_string,
        dbms_redis::api::rpush,
        dbms_redis::api::sadd,
        dbms_redis::api::zadd,
        dbms_redis::api::hmset,
    ]);
    app.run(tauri::generate_context!())
        .expect("error while running ahridbms application");
    Ok(())
}
