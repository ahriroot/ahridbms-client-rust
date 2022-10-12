pub mod api;
pub mod entity;
pub mod utils;

use tauri::{
    plugin::{Builder, TauriPlugin},
    Runtime,
};

/// Initializes the plugin.
pub fn init<R: Runtime>() -> TauriPlugin<R> {
    let mut plugin = Builder::new("redis");
    plugin = plugin.invoke_handler(tauri::generate_handler![
        api::info,
        api::keys,
        api::get,
        api::del,
        api::expire,
        api::exec,
        api::string::set,
        api::string::reset,
        api::list::lpush,
        api::list::rpush,
        api::list::lpop,
        api::list::rpop,
        api::list::lset,
        api::set::sadd,
        api::set::srem,
        api::zset::zadd,
        api::zset::zrem,
        api::hash::hset,
        api::hash::hdel,
        api::json::json_set,
        api::test,
    ]);
    plugin.build()
}
