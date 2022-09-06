import { InvokeArgs } from "@tauri-apps/api/tauri"
import request from "./request"


interface ResetArgs extends InvokeArgs {
    conn: any
    key: string
    db: string
}

const reset = async (params: ResetArgs): Promise<any> => {
    let res = await request<any>('plugin:redis|reset', params)
    return res
}

export { reset }
