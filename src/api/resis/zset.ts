import { InvokeArgs } from "@tauri-apps/api/tauri"
import request from "./request"


interface ZaddArgs extends InvokeArgs {
    conn: any
    key: string
    value: {
        score: number
        value: string
    }[]
    ttl: number
    db: string
}

interface ZremArgs extends InvokeArgs {
    conn: any
    key: string
    value: string[]
    db: string
}

const zadd = async (params: ZaddArgs): Promise<number> => {
    let res = await request<number>('plugin:redis|zadd', params)
    return res
}

const zrem = async (params: ZremArgs): Promise<number> => {
    let res = await request<number>('plugin:redis|zrem', params)
    return res
}

export { zadd, zrem }
