#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use ahridbms::dbms_redis;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut app = tauri::Builder::default();
    app = app.invoke_handler(tauri::generate_handler![dbms_redis::keys, dbms_redis::get, dbms_redis::key_space]);
    app.run(tauri::generate_context!())
        .expect("error while running ahridbms application");
    Ok(())
}
