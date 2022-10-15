#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

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
    let drives = vec!["test", "redis", "postgres", "mongodb"];
    for drive in drives {
        match drive {
            "test" => {
                use plugin_test;
                app = app.plugin(plugin_test::init());
            }
            "redis" => {
                use plugin_redis;
                app = app.plugin(plugin_redis::init());
            }
            "postgres" => {
                use plugin_postgres;
                app = app.plugin(plugin_postgres::init());
            }
            "mongodb" => {
                use plugin_mongodb;
                app = app.plugin(plugin_mongodb::init());
            }
            _ => {}
        }
    }

    app = app.invoke_handler(tauri::generate_handler![
        close_splashscreen,
    ]);
    app.run(tauri::generate_context!())
        .expect("error while running ahridbms application");
    Ok(())
}
