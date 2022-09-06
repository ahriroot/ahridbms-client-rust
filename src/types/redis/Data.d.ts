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

export interface Response<T> {
    code: number
    msg: string
    data: T
}

export interface INewFieldValue {
    string: {
        key: string
        value: string
        ttl: string
    },
    list: {
        key: string
        value: {
            value: string
        }[]
        ttl: string
    },
    set: {
        key: string
        value: {
            value: string
        }[]
        ttl: string
    },
    zset: {
        key: string
        value: {
            score: number
            value: string
        }[]
        ttl: string
    },
    hash: {
        key: string
        value: {
            field: string
            value: string
        }[]
        ttl: string
    }
}
