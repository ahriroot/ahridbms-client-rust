import { InvokeArgs } from "@tauri-apps/api/tauri"
import request from "./request"


interface PushArgs extends InvokeArgs {
    conn: any
    key: string
    value: string[]
    ttl: number
    db: string
}

interface PopArgs extends InvokeArgs {
    conn: any
    key: string
    db: string
}

interface LsetArgs extends InvokeArgs {
    conn: any
    key: string
    index: number
    value: string
    db: string
}

const lpush = async (params: PushArgs): Promise<number> => {
    let res = await request<number>('plugin:redis|lpush', params)
    return res
}

const rpush = async (params: PushArgs): Promise<number> => {
    let res = await request<number>('plugin:redis|rpush', params)
    return res
}

const lpop = async (params: PopArgs): Promise<string> => {
    let res = await request<string>('plugin:redis|lpop', params)
    return res
}

const rpop = async (params: PopArgs): Promise<string> => {
    let res = await request<string>('plugin:redis|rpop', params)
    return res
}

const lset = async (params: LsetArgs): Promise<string> => {
    let res = await request<string>('plugin:redis|lset', params)
    return res
}

export { lpush, rpush, lpop, rpop, lset }

