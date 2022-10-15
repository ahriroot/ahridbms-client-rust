pub mod api;
pub mod entity;
use tauri::{
    plugin::{Builder, TauriPlugin},
    Runtime,
};

/// Initializes the plugin.
pub fn init<R: Runtime>() -> TauriPlugin<R> {
    let mut plugin = Builder::new("mongodb");
    plugin = plugin.invoke_handler(tauri::generate_handler![api::find, api::test]);
    plugin.build()
}

#[cfg(test)]
mod tests {
    use futures_util::StreamExt;
    use mongodb::{
        bson::{doc, Document},
        options::ClientOptions,
        Client, Collection,
    };

    #[tokio::test]
    async fn test1() {
        let mut client_options = ClientOptions::parse("mongodb://root:Aa12345.@localhost:27017")
            .await
            .unwrap();

        // Manually set an option.
        client_options.app_name = Some("AhriDMBS App".to_string());

        // Get a handle to the deployment.
        let client = Client::with_options(client_options).unwrap();

        // ping
        let res = client
            .database("admin")
            .run_command(doc! {"ping": 1}, None)
            .await
            .unwrap();
        println!("{:?}", res);

        assert_eq!(1, 1);
    }
}
