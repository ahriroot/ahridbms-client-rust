pub mod api;
pub mod entity;
use tauri::{
    plugin::{Builder, TauriPlugin},
    Runtime,
};

/// Initializes the plugin.
pub fn init<R: Runtime>() -> TauriPlugin<R> {
    let mut plugin = Builder::new("mongodb");
    plugin = plugin.invoke_handler(tauri::generate_handler![
        api::databases,
        api::collections,
        api::documents,
        api::insert_one,
        api::insert_many,
        api::update_one,
        api::update_many,
        api::delete_one,
        api::delete_many,
        api::find,
        api::drop_database,
        api::drop_collection,
        api::test
    ]);
    plugin.build()
}

#[cfg(test)]
mod tests {

    #[tokio::test]
    async fn test() {
        assert_eq!(1, 1);
    }
}
