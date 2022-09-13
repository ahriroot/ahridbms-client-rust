pub mod api;
pub mod entity;

use tauri::{
    plugin::{Builder, TauriPlugin},
    Runtime,
};

/// Initializes the plugin.
pub fn init<R: Runtime>() -> TauriPlugin<R> {
    Builder::new("postgres").build()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test1() {
        let res = api::pg_get_databases(entity::Connection {
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
