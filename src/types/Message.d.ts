import Connection from './Connection'

export interface OpenTabMesagae<T> {
    id: string
    conn: Connection
    tab_type: string
    data: T
}