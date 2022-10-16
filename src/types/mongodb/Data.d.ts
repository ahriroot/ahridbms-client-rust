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
