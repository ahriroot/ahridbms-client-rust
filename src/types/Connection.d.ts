import { RedisConnect } from './redis'

interface Connection {
    id: string
    db_type: string,
    info: RedisConnect
}

export { Connection }