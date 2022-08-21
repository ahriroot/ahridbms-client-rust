import Connection from './Connection'

export interface OpenTabMesagae {
    id: string
    conn: Connection
    tab_type: string
}