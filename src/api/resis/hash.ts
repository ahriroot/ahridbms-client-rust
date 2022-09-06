import { InvokeArgs } from "@tauri-apps/api/tauri"
import request from "./request"


interface HsetArgs extends InvokeArgs {
    conn: any
    key: string
    value: {
        field: string
        value: string
    }[]
    ttl: number
    db: string
}

interface HdeltArgs extends InvokeArgs {
    conn: any
    key: string
    fields: string[]
    db: string
}

const hset = async (params: HsetArgs): Promise<string> => {
    let res = await request<string>('plugin:redis|hset', params)
    return res
}

const hdel = async (params: HdeltArgs): Promise<string> => {
    let res = await request<string>('plugin:redis|hdel', params)
    return res
}

export { hset, hdel }

