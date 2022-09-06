import { InvokeArgs } from "@tauri-apps/api/tauri"
import request from "./request"


interface SaddArgs extends InvokeArgs {
    conn: any
    key: string
    value: string[]
    ttl: number
    db: string
}

interface SremArgs extends InvokeArgs {
    conn: any
    key: string
    value: string[]
    db: string
}

const sadd = async (params: SaddArgs): Promise<number> => {
    let res = await request<number>('plugin:redis|sadd', params)
    return res
}

const srem = async (params: SremArgs): Promise<number> => {
    let res = await request<number>('plugin:redis|srem', params)
    return res
}

export { sadd, srem }

