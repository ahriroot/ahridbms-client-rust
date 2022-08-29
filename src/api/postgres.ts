import { invoke, InvokeArgs } from "@tauri-apps/api/tauri"
import { Response } from '@/types/redis'
import { Connection } from "@/types/Connection"
import { PostgresConnect } from "@/types/postgres"
import ApiError from "@/types/error"


interface PgGetDatabasesArgs extends InvokeArgs {
    conn: Connection<PostgresConnect>
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
    return res.data
}

const pgGetDatabases = async (params: PgGetDatabasesArgs): Promise<any | ApiError> => {
    let res = await request<Response<any>>('pg_get_databases', params)
    return res
}

export { pgGetDatabases }

