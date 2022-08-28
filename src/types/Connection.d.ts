import { RedisConnect } from './redis'
import { PostgresConnect } from './postgres'

interface Connection<T> {
    id: string
    db_type: string,
    info: T
}  // RedisConnect | PostgresConnect

export { Connection }