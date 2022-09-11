#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use ahridbms;
use tauri::Manager;

#[tauri::command]
async fn close_splashscreen(window: tauri::Window) {
    // Close splashscreen
    if let Some(splashscreen) = window.get_window("splashscreen") {
        splashscreen.close().unwrap();
    }
    // Show main window
    window.get_window("main").unwrap().show().unwrap();
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut app = tauri::Builder::default();

    // 循环 drives
    let drives = vec!["redis"];
    for drive in drives {
        match drive {
            "test" => {
                use tauri_plugin_test;
                app = app.plugin(tauri_plugin_test::init());
            }
            "redis" => {
                use tauri_plugin_redis;
                app = app.plugin(tauri_plugin_redis::init());
            }
            _ => {}
        }
    }
    
    app = app.invoke_handler(tauri::generate_handler![
        close_splashscreen,
        // postgres
        ahridbms::dbms_postgres::api::pg_select,
        ahridbms::dbms_postgres::api::pg_get_databases,
    ]);
    app.run(tauri::generate_context!())
        .expect("error while running ahridbms application");
    Ok(())
}
