use tauri::{
    plugin::{Builder, TauriPlugin},
    Runtime,
};

/// Initializes the plugin.
pub fn init<R: Runtime>() -> TauriPlugin<R> {
    println!("Load plugin tauri-plugin-test!");
    Builder::new("test").build()
}
