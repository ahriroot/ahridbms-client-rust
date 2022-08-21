import RedisConnect from './redis/Conn.ts'

interface Connection {
    id: string
    db_type: string,
    info: RedisConnect
}

export { Connection, RedisConnect }