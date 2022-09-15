import { invoke, InvokeArgs } from "@tauri-apps/api/tauri"
import { Response } from '@/types/redis'
import { Connection } from "@/types/Connection"
import { PostgresConnect } from "@/types/postgres"
import ApiError from "@/types/error"


interface GetDatabasesArgs extends InvokeArgs {
    conn: Connection<PostgresConnect>
}

interface GetTablesArgs extends InvokeArgs {
    conn: Connection<PostgresConnect>
    database: string
}

interface GetColumnsArgs extends InvokeArgs {
    conn: Connection<PostgresConnect>
    database: string
    table: string
}

interface SelectArgs extends InvokeArgs {
    conn: Connection<PostgresConnect>
    skip: number
    limit: number
    page: number
    size: number
    database: string
    table: string
}

interface UpdateArgs extends InvokeArgs {
    conn: Connection<PostgresConnect>
    database: string
    sql: string
}

const request = async <T>(command: string, params: any): Promise<T | ApiError> => {
    let res = await invoke<Response<T>>(command, params)
    if (res.code !== 10000) {
        if (res.code < 50000) {
            window.$message.warning(res.msg)
        } else {
            window.$message.error(res.msg)
        }
        return Promise.resolve({ is_error: true, code: res.code, msg: res.msg, from: '' })
    }
    return res.data.Success
}

const getDatabases = async (params: GetDatabasesArgs): Promise<any | ApiError> => {
    let res = await request<Response<any>>('plugin:postgres|get_databases', params)
    return res
}

const getTables = async (params: GetTablesArgs): Promise<any | ApiError> => {
    let res = await request<Response<any>>('plugin:postgres|get_tables', params)
    return res
}

const getColumns = async (params: GetColumnsArgs): Promise<any | ApiError> => {
    let res = await request<Response<any>>('plugin:postgres|get_columns', params)
    return res
}

const select = async (params: SelectArgs): Promise<any | ApiError> => {
    let res = await request<Response<any>>('plugin:postgres|select', params)
    return res
}

const update = async (params: UpdateArgs): Promise<any | ApiError> => {
    let res = await request<Response<any>>('plugin:postgres|update', params)
    return res
}

export { getDatabases, getTables, getColumns, select, update }

