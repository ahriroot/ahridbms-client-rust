export interface Keyvalue {
    key: string
    key_type: string
    value: string
    size: number
    ttl: number
}

export interface AllKeys {
    [key: string]: Keyvalue
}
