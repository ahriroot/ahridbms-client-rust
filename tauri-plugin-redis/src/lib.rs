pub mod api;
pub mod entity;

use tauri::{
    plugin::{Builder, TauriPlugin},
    Runtime,
};

/// Initializes the plugin.
pub fn init<R: Runtime>() -> TauriPlugin<R> {
    let mut plugin = Builder::new("redis");
    plugin = plugin.invoke_handler(tauri::generate_handler![
        api::keys,
        api::get,
        api::del,
        api::expire,
        api::key_space,
        api::set_string,
        api::rpush,
        api::sadd,
        api::zadd,
        api::srem,
        api::hmset,
    ]);
    plugin.build()
}
