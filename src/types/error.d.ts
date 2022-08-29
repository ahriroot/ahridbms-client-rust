export default interface ApiError {
    is_error: boolean
    code?: number
    msg?: string
    from?: any
}