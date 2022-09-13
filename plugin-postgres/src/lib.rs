pub mod api;
pub mod entity;

use tauri::{
    plugin::{Builder, TauriPlugin},
    Runtime,
};

/// Initializes the plugin.
pub fn init<R: Runtime>() -> TauriPlugin<R> {
    let mut plugin = Builder::new("postgres");
    plugin = plugin.invoke_handler(tauri::generate_handler![
        api::get_databases,
        api::get_tables,
        api::get_columns,
    ]);
    plugin.build()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test1() {
        let res = api::get_databases(entity::Connection {
            id: "postgres".to_string(),
            db_type: "postgres".to_string(),
            info: entity::Info {
                name: "postgres".to_string(),
                host: "localhost".to_string(),
                port: "5432".to_string(),
                user: "postgres".to_string(),
                pass: "Aa12345.".to_string(),
                db: "postgres".to_string(),
            },
        })
        .await;
        println!("{:#?}", res);
        assert_eq!(1, 1);
    }
}
