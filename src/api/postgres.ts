import { invoke, InvokeArgs } from "@tauri-apps/api/tauri"
import { AllKeys, Response } from '@/types/redis'
import { Connection } from "@/types/Connection"
import { PostgresConnect } from "@/types/postgres"


interface PgGetDatabasesArgs extends InvokeArgs {
    conn: Connection<PostgresConnect>
}
const request = async <T>(command: string, params: any): Promise<T> => {
    let res = await invoke<Response<T>>(command, params)
    if (res.code !== 10000) {
        if (res.code < 50000) {
            window.$message.warning(res.msg)
        } else {
            window.$message.error(res.msg)
        }
        return Promise.reject(res.msg)
    }
    return res.data
}

const pgGetDatabases = async (params: PgGetDatabasesArgs): Promise<any> => {
    let res = await request<Response<any>>('pg_get_databases', params)
    return res
}

export { pgGetDatabases }

