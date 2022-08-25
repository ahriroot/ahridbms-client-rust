import RedisConnect from './Conn'
import { Keyvalue, AllKeys, Response, INewFieldValue } from './Data'

interface Connection {
    id: string
    db_type: string,
    info: RedisConnect
}

export { Connection, RedisConnect, Keyvalue, AllKeys, Response, INewFieldValue }