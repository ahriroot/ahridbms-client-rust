import { InvokeArgs } from "@tauri-apps/api/tauri"
import request from "./request"


interface JsonSetArgs extends InvokeArgs {
    conn: any
    key: string
    path?: string
    value: string
    ttl: number
    db: string
}

const json_set = async (params: JsonSetArgs): Promise<string> => {
    if (params.path === undefined || params.path === '') {
        params.path = "$"
    }
    let res = await request<string>('plugin:redis|json_set', params)
    return res
}

export { json_set }

