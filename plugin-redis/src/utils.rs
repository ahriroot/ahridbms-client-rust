use crate::entity::Connection;
use redis;

pub async fn get_connection(conn: Connection, db: String) -> redis::RedisResult<redis::Connection> {
    let conn_str = format!(
        "redis://{}:{}@{}:{}/{}",
        "", conn.info.pass, conn.info.host, conn.info.port, db
    );

    let client = redis::Client::open(conn_str)?;

    let conn = client.get_connection();
    conn
}
