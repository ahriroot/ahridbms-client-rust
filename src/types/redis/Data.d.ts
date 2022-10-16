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

export interface IExecValueNil {
    Nil: 'Nil'
}

export interface IExecValueOkay {
    Okay: 'Okay'
}

export interface IExecValueData {
    Data: Uint8Array
}

export interface IExecValueStatus {
    Status: string
}

export interface IExecValueInteger {
    Integer: number
}

export interface IExecValueBulk {
    Bulk: IExecValueData[]
}

export interface IExecValueError {
    Error: string
}

export interface IExecType {
    nil: "Nil"
    okay: "Okay"
    data: "Data"
    status: "Status"
    integer: "Integer"
    bulk: "Bulk"
    error: "Error"
}

export interface IExecResult {
    command: string
    type_: "Nil" | "Okay" | "Data" | "Status" | "Integer" | "Bulk" | "Error"
    value: IExecValueNil | IExecValueOkay | IExecValueData | IExecValueStatus | IExecValueInteger | IExecValueBulk | IExecValueError
}

export interface Success<T> {
    Success: T
}

export interface Error<T> {
    Success: T
}

export interface Error5<T> {
    Success: T
}

export interface Response<T> {
    code: number
    msg: string
    data: Success<T> | Success<T> | Success<T>
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
    },
    json: {
        key: string
        value: string
        ttl: string
    }
}
