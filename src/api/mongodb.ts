import { invoke, InvokeArgs } from "@tauri-apps/api/tauri"
import { Connection } from "@/types/Connection"
import { PostgresConnect } from "@/types/postgres"
import { ResponseMongodb } from "@/types/redis/Data"


const request = async <T>(command: string, params: any): Promise<T> => {
    let res = await invoke<ResponseMongodb<T>>(command, params)
    if (res.code !== 10000) {
        if (res.code < 50000) {
            window.$message.warning(res.msg, {
                closable: true,
                duration: 10000
            })
        } else {
            window.$message.error(res.msg)
        }
        throw new Error(res.msg)
    }
    return res.data
}


interface TestArgs extends InvokeArgs {
    conn: Connection<PostgresConnect>
    database: string
}

const test = async (params: TestArgs): Promise<any> => {
    let res = await request<any>('plugin:mongodb|test', params)
    return res
}

export { test }

