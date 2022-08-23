import RedisConnect from './Conn'
import { Keyvalue, AllKeys } from './Data'

interface Connection {
    id: string
    db_type: string,
    info: RedisConnect
}

export { Connection, RedisConnect, Keyvalue, AllKeys }