import { invoke, InvokeArgs } from "@tauri-apps/api/tauri"
import { AllKeys, Response } from '@/types/redis'


interface SetStringArgs extends InvokeArgs {
    conn: any
    key: string
    value: string
    ttl: number
}

interface RPushArgs extends InvokeArgs {
    conn: any
    key: string
    value: string[]
    ttl: number
}

interface SaddArgs extends InvokeArgs {
    conn: any
    key: string
    value: string[]
    ttl: number
}

interface ZaddArgs extends InvokeArgs {
    conn: any
    key: string
    value: {
        score: number
        value: string
    }[]
    ttl: number
}

interface HmsetArgs extends InvokeArgs {
    conn: any
    key: string
    value: {
        key: string
        value: string
    }[]
    ttl: number
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

const keys = async (params: InvokeArgs): Promise<AllKeys[]> => {
    let res = await request<AllKeys[]>('keys', params)
    return res
}

const setString = async (params: SetStringArgs): Promise<string> => {
    let res = await request<string>('set_string', params)
    return res
}

const rpush = async (params: RPushArgs): Promise<number> => {
    let res = await request<number>('rpush', params)
    return res
}

const sadd = async (params: SaddArgs): Promise<number> => {
    let res = await request<number>('sadd', params)
    return res
}

const zadd = async (params: ZaddArgs): Promise<number> => {
    let res = await request<number>('zadd', params)
    return res
}

const hmset = async (params: HmsetArgs): Promise<string> => {
    let res = await request<string>('hmset', params)
    return res
}

export { keys, setString, rpush, sadd, zadd, hmset }

